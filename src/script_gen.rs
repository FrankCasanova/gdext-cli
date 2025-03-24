//third-party modules
use anyhow::Result;
use heck::ToUpperCamelCase;
use std::fs;

/// Generates a new Rust source file for a Godot node with the given name and node type.
/// 
/// # Arguments
///
/// * `name` - The name of the script, which will be converted to snake_case for the filename and PascalCase for the struct.
/// * `typenode` - The Godot node type to base the generated struct on.
///
/// # Returns
///
/// * `Result<()>` - Returns an Ok result if the script file is successfully created, or an Err if an error occurs.
///
/// # Description
///
/// This function ensures the `src` directory exists, converts the provided script name into appropriate cases for the
/// Rust module and struct, and generates a Rust source file implementing a Godot class based on the specified node type. 
/// The generated code includes necessary imports, struct definition, and implementation of the Godot class interface.
/// The function also updates `lib.rs` to include the newly created module.

pub fn handle_script(name: &str, typenode: &str) -> Result<()> {
    // Ensure src directory exists
    fs::create_dir_all("src")?;

    // Convert name to appropriate cases (only for the Rust struct/module)
    let mod_name: String = name.to_lowercase();       // snake_case for file/mod
    let struct_name: String = name.to_upper_camel_case(); // PascalCase for struct

    // Use the node type exactly as provided by user
    let node_type: &str = typenode;
    let interface_name: String = format!("I{}", node_type);

    // Generate scene code
    let code = format!(
        r#"use godot::prelude::*;
use godot::classes::{node_type};
use godot::classes::{interface_name};

#[derive(GodotClass)]
#[class(base={node_type})]
pub struct {struct_name} {{
    base: Base<{node_type}>
}}

#[godot_api]
impl {interface_name} for {struct_name} {{
    fn init(base: Base<{node_type}>) -> Self {{
        godot_print!("{struct_name} initialized");
        Self {{ base }}
    }}
}}
"#,
        node_type = node_type,
        interface_name = interface_name,
        struct_name = struct_name
    );

    // Write to file
    let filename: String = format!("src/{}.rs", mod_name);
    fs::write(&filename, code)?;

    // Add mod declaration to top of lib.rs
    let lib_path: &str = "src/lib.rs";
    let content: String = fs::read_to_string(lib_path)?;
    
    let mod_line: String = format!("mod {};\n", mod_name);
    
    if !content.contains(&mod_line) {
        // Insert after use statements
        let use_end: usize = content.find("use godot::prelude::*;\n")
            .map(|pos| pos + "use godot::prelude::*;\n".len())
            .unwrap_or(0);

        let (before, after) = content.split_at(use_end);
        let new_content: String = format!("{}{}{}", before, mod_line, after);
        fs::write(lib_path, new_content)?;
    }

    Ok(())
}