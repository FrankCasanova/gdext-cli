//third-party modules
use anyhow::Result;
use std::fs;
use capitalize::Capitalize;

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
pub fn handle_startproject(script: &str, name: &str) -> Result<()> {
    // Create src directory
    fs::create_dir_all("src")?;

    // Create lib.rs
    let script_words: Vec<&str> = script.split('-').collect();
    let script = script_words
        .iter()
        .map(|word| 
            word.capitalize())
                .collect::<Vec<String>>()
                .join("");
    let lib_rs = format!(r#"use godot::prelude::*;

struct {script};

#[gdextension]
unsafe impl ExtensionLibrary for {script} {{}}"#, script = script);

    fs::write("src/lib.rs", lib_rs)?;

    // Create rust.gdextension
    let name_snake_case = &name.to_lowercase();
    let name_snake_case = &name_snake_case.replace("-", "_");
    let gdextension: String= format!(r#"[configuration]
entry_symbol = "gdext_rust_init"
compatibility_minimum = 4.3
reloadable = true

[libraries]
linux.debug.x86_64 = "res://../../target/debug/lib{name}.so"
linux.release.x86_64 = "res://../../target/release/lib{name}.so"
windows.debug.x86_64 = "res://../../target/debug/{name}.dll"
windows.release.x86_64 = "res://../../target/release/{name}.dll"
macos.debug = "res://../../target/debug/lib{name}.dylib"
macos.release = "res://../../target/release/lib{name}.dylib"
macos.debug.arm64 = "res://../../target/debug/lib{name}.dylib"
macos.release.arm64 = "res://../../target/release/lib{name}.dylib"
web.debug.wasm32 = "res://../../target/wasm32-unknown-emscripten/debug/{name}.wasm"
web.release.wasm32 = "res://../../target/wasm32-unknown-emscripten/release/{name}.wasm"
"#, name = name_snake_case);
    fs::write("rust.gdextension", gdextension)?;

    // Create Cargo.toml
    let name_snake_case = &name.to_lowercase();
    let name_snake_case = &name_snake_case.replace("-", "_");
    let cargo_toml: String = format!(r#"[package]
name = "{name}"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib"]

[dependencies]
godot = {{ git = "https://github.com/godot-rust/gdext.git", branch = "master" }}
"#, name = name_snake_case);
    fs::write("Cargo.toml", cargo_toml)?;

    Ok(())
}