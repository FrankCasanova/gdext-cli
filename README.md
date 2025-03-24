

# gdext-cli

A CLI tool to generate Godot-Rust projects and scenes.

## Overview

`gdext-cli` simplifies the process of creating and managing Godot game projects that use Rust as their scripting language via [godot-rust](https://github.com/godot-rust/gdext). It automates common tasks like project initialization and scene creation.

## Features

- **Project Initialization**: Quickly bootstrap a new Godot-Rust project with the necessary directory structure and configuration files
- **Script BoilerPlate Generation**: Generate Rust code for new scenes with the appropriate Godot node types

## Installation

```bash
#from source
git clone https://github.com/FrankCasanova/gdext-cli.git
cd gdext-cli
cargo install --path .
```

## Usage

### Initialize a new Godot-Rust project

```bash
gdext-cli startproject
```

This will create the necessary directories and files for a new Godot-Rust project.

### Create a new scene

```bash
gdext-cli scene --name player --typenode CharacterBody2D
```

This command generates a new script with:
- A file named `player.rs` in the `src` directory
- A Rust struct named `Player` that extends the `CharacterBody2D` node type

## Node Type Format Requirements

**Important:** When specifying the `--typenode` parameter, you must use the exact PascalCase format as used in the Godot API. The CLI cannot reliably determine the correct capitalization for all node types due to Godot's specific naming conventions.

For example:
- Use `CharacterBody2D` (not `characterbody2d` or `Character_Body_2D`)
- Use `RigidBody3D` (not `rigidbody3d` or `Rigidbody3D`)
- Use `Control` (not `control`)
- Use `AnimationPlayer` (not `animationplayer` or `Animation_Player`)

Incorrect capitalization will result in compilation errors when your Rust code attempts to reference the Godot types.

For a complete list of Godot node types with their correct capitalization, refer to the [official Godot documentation](https://docs.godotengine.org/en/stable/classes/index.html).


## Command Reference

```
USAGE:
    gdext-cli [SUBCOMMAND]

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    startproject    Initialize a new Godot-Rust project
    scene           Generate a new scene with the given name and node type
    help            Print this message or the help of the given subcommand(s)
```

### Scene Command

```
USAGE:
    gdext-cli scene [OPTIONS] --name <NAME> --typenode <TYPENODE>

OPTIONS:
    --name <NAME>          The name of the scene (snake_case for file, PascalCase for struct)
    --typenode <TYPENODE>  The Godot node type (e.g., CharacterBody3D)
```

## Examples

### Creating a player character

```bash
gdext-cli scene player CharacterBody2D
```

### Creating a UI element

```bash
gdext-cli scene main_menu Control
```

## License

[MIT License](LICENSE)

## Author

Frank Casanova
- Email: frankcasanova.info@gmail.com
- GitHub: [FrankCasanova](https://github.com/FrankCasanova)
- LinkedIn: [frankcasanova](https://linkedin.com/in/frankcasanova-)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.