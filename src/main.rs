//third-party modules
use anyhow::Result;
use clap::{Parser, Subcommand};

//own modules
use script_gen::handle_script;
mod script_gen;
use startproject_gen::handle_startproject;
mod startproject_gen;
use template_gen::handle_template;
mod template_gen;


#[derive(Parser)]
#[command(name = "gdext-cli")]
#[command(about = "A CLI tool to generate Godot-Rust projects", long_about = None)]
#[command(version)]
#[command(author = "Frank Casanova\n<frankcasanova.info@gmail.com>\n<https://github.com/FrankCasanova>\n<https://linkedin.com/in/frankcasanova->")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new Godot-Rust project
    Startproject{
        script: String,
        name: String,
    },
    /// Generate a new script with the given name and node type
    Script {
        /// The name of the script (snake_case for file, PascalCase for struct)
        name: String,
        /// The Godot node type (e.g., CharacterBody3D)
        typenode: String,
    },
    /// Create a new project from a template
    Template{
        name: String,
    },
}

/// The entry point of the CLI tool.
///
/// Parses the command line arguments and calls either
/// `handle_startproject` or `handle_script` depending on the
/// subcommand selected.
fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Startproject {script,name} => handle_startproject(&script, &name),
        Commands::Script { name, typenode } => handle_script(&name, &typenode),
        Commands::Template {name } => handle_template(&name),
    }
}



