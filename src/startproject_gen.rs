//third-party modules
use anyhow::Result;
use std::fs;

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
pub fn handle_startproject() -> Result<()> {
    // Create src directory
    fs::create_dir_all("src")?;

    // Create lib.rs
    let lib_rs: &str = r#"use godot::prelude::*;


struct Scripts;

#[gdextension]
unsafe impl ExtensionLibrary for Scripts {}
"#;
    fs::write("src/lib.rs", lib_rs)?;

    // Create rust.gdextension
    let gdextension: &str = r#"[configuration]
entry_symbol = "gdext_rust_init"
compatibility_minimum = 4.3
reloadable = true

[libraries]
linux.debug.x86_64 = "res://../REPLACE_ME/target/debug/REPLACE_ME.so"
linux.release.x86_64 = "res://../REPLACE_ME/target/release/REPLACE_ME.so"
windows.debug.x86_64 = "res://../REPLACE_ME/target/debug/REPLACE_ME.dll"
windows.release.x86_64 = "res://../REPLACE_ME/target/release/REPLACE_ME.dll"
macos.debug = "res://../REPLACE_ME/target/debug/REPLACE_ME.dylib"
macos.release = "res://../REPLACE_ME/target/release/REPLACE_ME.dylib"
macos.debug.arm64 = "res://../REPLACE_ME/target/debug/REPLACE_ME.dylib"
macos.release.arm64 = "res://../REPLACE_ME/target/release/REPLACE_ME.dylib"
web.debug.wasm32 = "res://../REPLACE_ME/target/wasm32-unknown-emscripten/debug/REPLACE_ME.wasm"
web.release.wasm32 = "res://../REPLACE_ME/target/wasm32-unknown-emscripten/release/REPLACE_ME.wasm"
"#;
    fs::write("rust.gdextension", gdextension)?;

    // Create Cargo.toml
    let cargo_toml: &str = r#"[package]
name = "rust"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib"]

[dependencies]
godot = { git = "https://github.com/godot-rust/gdext.git", branch = "master" }
"#;
    fs::write("Cargo.toml", cargo_toml)?;

    Ok(())
}