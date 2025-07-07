//third-party modules
use anyhow::Result;

//own modules
use crate::startproject_gen::handle_startproject;
use crate::script_gen::handle_script;

/// Creates a new project from a template.
///
/// # Description
///
/// This function initializes a new Godot-Rust project and creates a default
/// player script.
///
/// # Errors
///
/// If the project directory already exists, this function will return an
/// error. If the files cannot be created, this function will return an
/// error. If the `godot-rust` crate cannot be found, this function will
/// return an error.
pub fn handle_template(name: &str) -> Result<()> {
    handle_startproject(name, name)?;
    handle_script("player", "character-body-2d")?;
    Ok(())
}