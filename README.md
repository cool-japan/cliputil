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

Cliputil requires the following components to be installed on your system:

### Core Requirements
- Rust (latest stable version)
- Cargo (comes with Rust)

### Language-specific Requirements

#### Python
- Python 3.x
- Black (for formatting)

#### JavaScript
- Node.js
- Prettier (for formatting)

#### Rust
- Rust toolchain
- rustfmt (usually comes with Rust)

#### C++
- G++ (GNU C++ Compiler)
- ClangFormat (for formatting)

#### Java
- Java Development Kit (JDK)
- google-java-format (for formatting)

#### PHP
- PHP
- PHP CS Fixer (for formatting)

### Installation Instructions

#### On Linux (Ubuntu/Debian)
```bash
# Core
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Python
sudo apt install python3 python3-pip
pip3 install black

# JavaScript
sudo apt install nodejs npm
npm install -g prettier

# C++
sudo apt install g++ clang-format

# Java
sudo apt install default-jdk
wget https://github.com/google/google-java-format/releases/download/v1.15.0/google-java-format-1.15.0-all-deps.jar
sudo mv google-java-format-1.15.0-all-deps.jar /usr/local/bin/google-java-format.jar
echo 'alias google-java-format="java -jar /usr/local/bin/google-java-format.jar"' >> ~/.bashrc

# PHP
sudo apt install php-cli
wget https://cs.symfony.com/download/php-cs-fixer-v3.phar -O php-cs-fixer
chmod a+x php-cs-fixer
sudo mv php-cs-fixer /usr/local/bin/php-cs-fixer
```

#### On MacOS
```bash
# Core
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Homebrew if not already installed
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Python
brew install python
pip3 install black

# JavaScript
brew install node
npm install -g prettier

# C++
brew install gcc
brew install clang-format

# Java
brew install openjdk
brew install google-java-format

# PHP
brew install php
brew install php-cs-fixer
```

#### On Windows
1. Install Rust from https://www.rust-lang.org/tools/install
2. Install Python from https://www.python.org/downloads/
3. Install Node.js from https://nodejs.org/
4. Install Visual Studio with C++ support
5. Install JDK from https://adoptopenjdk.net/
6. Install PHP from https://windows.php.net/download/

Then, using PowerShell:
```powershell
# Python formatter
pip install black

# JavaScript formatter
npm install -g prettier

# Java formatter
Invoke-WebRequest -Uri "https://github.com/google/google-java-format/releases/download/v1.15.0/google-java-format-1.15.0-all-deps.jar" -OutFile "google-java-format.jar"

# PHP formatter
Invoke-WebRequest -Uri "https://cs.symfony.com/download/php-cs-fixer-v3.phar" -OutFile "php-cs-fixer"
```

Note: On Windows, you may need to add the installation directories to your PATH environment variable.
```

## License

This project is licensed under the Apache License, Version 2.0. See the [LICENSE](LICENSE) file for details.
