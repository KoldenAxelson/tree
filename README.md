# Tree Generator

A Rust-based command-line tool that generates a visual directory tree structure, similar to the `tree` command. Includes support for .gitignore rules and colorized output.

## Features

- 🎨 Colorized output:
  - Directories in cyan
  - Executable files in green
- 📝 Respects .gitignore rules
- 🚫 Automatically ignores .git directories
- 📂 Proper sorting (directories first, then files alphabetically)
- 🌲 Clean tree visualization with Unicode box-drawing characters

## Installation

1. Clone the repository
2. Build with cargo:
```bash
cargo build --release
```

## Usage

Run in current directory:
```bash
cargo run
```

Specify a directory:
```bash
cargo run -- /path/to/directory
```

## Example Output

```
my-project/
├── src/
│   ├── main.rs
│   └── lib.rs
├── tests/
│   └── integration_tests.rs
├── Cargo.toml
└── README.md
```

## Dependencies

- `ignore` - For .gitignore parsing
- `colored` - For terminal colors

## Contributing

Feel free to open issues or submit pull requests. Some ideas for improvements:
- Windows executable detection
- Custom ignore patterns
- File size information
- Last modified dates
- Additional file type colors
- Depth limiting
- Pattern matching

## License

MIT License