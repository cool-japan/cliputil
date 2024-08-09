# Cliputil

Cliputil is a versatile command-line utility for executing and formatting code from the clipboard. It supports multiple programming languages including Python, JavaScript, Rust, C++, Java, and PHP.

## Features

- Execute code directly from the clipboard
- Format code in the clipboard
- Support for multiple programming languages
- Option to output results to clipboard or terminal

## Installation

Ensure you have Rust and Cargo installed on your system. Then, clone this repository and build the project:

```bash
git clone https://github.com/cool-japan/cliputil.git
cd cliputil
cargo build --release
```

The executable will be available in `target/release/cliputil`.

## Usage

```bash
cliputil <command> [options]
```

Commands:
- `execute`: Execute code from clipboard
- `format`: Format code in clipboard

Options:
- `-l, --language <LANG>`: Specify the programming language
- `-v, --verbose`: Enable verbose output
- `-o, --output-clipboard`: Output result to clipboard

Example:
```bash
cliputil execute -l python
cliputil format -l javascript
```

## Supported Languages

- Python
- JavaScript
- Rust
- C++
- Java
- PHP

## Requirements

Ensure you have the necessary compilers and interpreters installed for the languages you wish to use:

- Python
- Node.js
- Rust
- G++
- Java Development Kit (JDK)
- PHP

## License

This project is licensed under the Apache License, Version 2.0. See the [LICENSE](LICENSE) file for details.
