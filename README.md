

# gdext-cli

A CLI tool to generate Godot-Rust projects and scripts.

## Overview

`gdext-cli` simplifies the process of creating and managing Godot game projects that use Rust as their scripting language via [godot-rust](https://github.com/godot-rust/gdext). It automates common tasks like project initialization and script creation.


## Features

- **Project Initialization**: Quickly bootstrap a new Godot-Rust project with the necessary directory structure and configuration files
- **Script BoilerPlate Generation**: Generate Rust code for new script to bind the appropriate Godot node types

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
gdext-cli startproject --script example-script --name example-game-name
```

This will create the necessary directories and files for a new Godot-Rust project.

### Create a new script

```bash
gdext-cli script --name player-name-example --typenode character-body-2d
```

This command generates a new script with:
- A file named `player.rs` in the `src` directory
- A Rust struct named `Player` that extends the `CharacterBody2D` node type

## Command Reference

```
USAGE:
    gdext-cli [SUBCOMMAND]

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    startproject    Initialize a new Godot-Rust project
    script           Generate a new script with the given name and node type
    help            Print this message or the help of the given subcommand(s)
```

### Script Command

```
USAGE:
    gdext-cli script [OPTIONS] --name <NAME> --typenode <TYPENODE>

OPTIONS:
    --name <NAME>          The name of the script (snake_case for file, PascalCase for struct)
    --typenode <TYPENODE>  The Godot node type (e.g., CharacterBody3D)
```

## Examples

### Creating a player character

```bash
gdext-cli script main-player character-body-2d
```

### Creating a UI element

```bash
gdext-cli script main-menu control
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
