

# gdext-cli

A CLI tool to generate Godot-Rust projects and scripts.

## Overview

`gdext-cli` simplifies the process of creating and managing Godot game projects that use Rust as their scripting language via [godot-rust](https://github.com/godot-rust/gdext). It automates common tasks like project initialization and script creation.


## Features

- **Project Initialization**: Quickly bootstrap a new Godot-Rust project with the necessary directory structure and configuration files
- **Script BoilerPlate Generation**: Generate Rust code for new script to bind the appropriate Godot node types

## Installation
First, clone and install the CLI tool:
```bash
git clone https://github.com/FrankCasanova/gdext-cli.git
```

```bash
cd gdext-cli
```

```bash
cargo install --path .
```

after that u can use the command `gdext-cli` to create a new project or script.
and also you can remove the cli folder if u dont want to use it anymore.

```bash
cd ..
```
```bash
rm -rf gdext-cli
```



## Usage

### Initialize a new Godot-Rust project

now! lets start a new project
first create a new folder for your game:
```bash
mkdir game_example
```
```bash
cd game_example
```
the most easy way to start a new project is using the command:
```bash
gdext-cli template name-of-ur-game
```
this command will generate a template example in the current directory like this:

```bash
game_example/
├── src/
│   ├── lib.rs
│   └── player.rs
├── Cargo.toml
└── rust.gdextension

```
just rename things and accomodate the starter project according to your needs.

but if u want to start a project with a script already created you can use the command:

```bash
gdext-cli startproject example-game example-game-name
```
This will create the necessary directories and files for a new Godot-Rust project.

### Create a new script

```bash
gdext-cli script player-name-example character-body-2d
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
