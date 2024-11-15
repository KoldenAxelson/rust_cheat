# Rust Cheat Sheet CLI

A command-line tool for quick Rust syntax reference. Access various Rust cheat sheets directly from your terminal with syntax highlighting.

## Installation

Clone and build from source:
```bash
git clone https://github.com/KoldenAxelson/rust_cheat.git
cd rust_cheat
cargo build --release
```

The binary will be available at `target/release/rust_cheat`
For ease-of-use, move it to `~/.bin/rust_cheat` or `~/.bin/crab`
Add this to your bash profile `export PATH="/Users/{your user}/.bin:${PATH}"`

## Usage

Show all available cheat sheets:
```bash
rust_cheat
```

Show a specific cheat sheet's outline (replace INDEX with sheet number):
```bash
rust_cheat INDEX
```

Show a specific section (replace INDEX with sheet number and SECTION with section number):
```bash
rust_cheat INDEX SECTION
```

Show full content of a cheat sheet:
```bash
rust_cheat INDEX 0
```

### Example

```bash
# Show all available cheat sheets
rust_cheat

# Show the outline of the basics sheet (index 0)
rust_cheat 0

# Show section 1 of the basics sheet
rust_cheat 0 1

# Show the entire basics sheet
rust_cheat 0 0
```

## Available Cheat Sheets

0. **Basics** - Fundamental Rust concepts including variables, control flow, functions, and more
1. **Intermediate** - Advanced patterns, traits, generics, lifetimes, and concurrency basics
2. **Advanced** - Unsafe Rust, advanced concurrency, macros, and more advanced topics
3. **Nalgebra** - Linear algebra operations using the nalgebra library

## Building from Source

Requirements:
- Rust toolchain (install via [rustup](https://rustup.rs/))

Build commands:
```bash
# Development build
cargo build

# Release build with optimizations
cargo build --release
```

## Contributing

Contributions are welcome! Here's how you can help:

1. Fork the repository
2. Create a new branch for your feature
3. Add or modify cheat sheets in the `src/sheets/` directory
4. Submit a pull request

When adding new cheat sheets:
- Use Rust-style comments (`// ----` for section dividers)
- Follow the section numbering format (`// 1. SECTION NAME`)
- Include clear, concise examples
- Add appropriate syntax highlighting

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Inspired by my Python Cheat Sheet CLI tool
- Uses [syntect](https://github.com/trishume/syntect) for syntax highlighting