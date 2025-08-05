//third-party modules
use anyhow::Result;
use capitalize::Capitalize;
use path_calculate::Calculate;
use std::{env, fs, path::PathBuf};

/// Initializes a new Godot-Rust project.
///
/// # Description
///
/// This function creates a new Godot-Rust project by creating the necessary
/// directories and files, including `src/lib.rs`, `rust.gdextension`, and
/// `Cargo.toml`. The project name is inferred from the current directory
/// name.
///
/// # Errors
///
/// If the project directory already exists, this function will return an
/// error. If the files cannot be created, this function will return an
/// error. If the `godot-rust` crate cannot be found, this function will
/// return an error.
pub fn handle_startproject(script: &str, name: &str, godot_dir: &Option<PathBuf>) -> Result<()> {
    // Create src directory
    fs::create_dir_all("src")?;

    // Setup directories
    let cwd = env::current_dir()?;
    let gdextension_filename = "rust.gdextension";
    let godot_project_dir = godot_dir.as_ref().map(|p| p.as_path()).unwrap_or(&cwd);

    // Use current working directory to build a relative path to godot project directory
    let binding = cwd.related_to(godot_project_dir)?;
    let gdextension_root = binding.to_str().unwrap();

    let mut gdextension_path = godot_project_dir.to_path_buf();
    gdextension_path.push(gdextension_filename);

    // Create lib.rs
    let lib_rs = generate_lib_rs(script);
    fs::write("src/lib.rs", lib_rs)?;

    // Create rust.gdextension
    let gdextension_content: String = generate_gdextension(name, gdextension_root);
    fs::write(gdextension_path, gdextension_content)?;

    // Create Cargo.toml
    let cargo_toml = generate_cargo_toml_file(name);
    fs::write("Cargo.toml", cargo_toml)?;

    Ok(())
}

fn generate_lib_rs(script: &str) -> String {
    // Convert the script name from kebab-case to PascalCase
    let script_words: Vec<&str> = script.split('-').collect();
    let script = script_words
        .iter()
        .map(|word| word.capitalize())
        .collect::<Vec<String>>()
        .join("");

    format!(
        r#"use godot::prelude::*;

struct {script};

#[gdextension]
unsafe impl ExtensionLibrary for {script} {{}}"#
    )
}

fn generate_gdextension(name: &str, godot_project_root: &str) -> String {
    // Create rust.gdextension
    let name_snake_case = &name.to_lowercase();
    let name_snake_case = &name_snake_case.replace("-", "_");
    let rust_target_path: PathBuf = [godot_project_root, "target"].iter().collect();

    format!(
        r#"[configuration]
entry_symbol = "gdext_rust_init"
compatibility_minimum = 4.3
reloadable = true

[libraries]
linux.debug.x86_64 = "res://{rust_project}/debug/lib{name}.so"
linux.release.x86_64 = "res://{rust_project}/release/lib{name}.so"
windows.debug.x86_64 = "res://{rust_project}/debug/{name}.dll"
windows.release.x86_64 = "res://{rust_project}/release/{name}.dll"
macos.debug = "res://{rust_project}/debug/lib{name}.dylib"
macos.release = "res://{rust_project}/release/lib{name}.dylib"
macos.debug.arm64 = "res://{rust_project}/debug/lib{name}.dylib"
macos.release.arm64 = "res://{rust_project}/release/lib{name}.dylib"
web.debug.wasm32 = "res://{rust_project}/wasm32-unknown-emscripten/debug/{name}.wasm"
web.release.wasm32 = "res://{rust_project}/wasm32-unknown-emscripten/release/{name}.wasm"
"#,
        name = name_snake_case,
        rust_project = rust_target_path.to_str().unwrap(),
    )
}

fn generate_cargo_toml_file(name: &str) -> String {
    let name_snake_case = &name.to_lowercase();
    let name_snake_case = &name_snake_case.replace("-", "_");
    format!(
        r#"[package]
name = "{name}"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib"]

[dependencies]
godot = {{ git = "https://github.com/godot-rust/gdext.git", branch = "master" }}
"#,
        name = name_snake_case
    )
}
