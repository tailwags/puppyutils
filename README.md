# Puppyutils

An efficient and correct implementation of coreutils, util-linux and other core
Linux system utilities written in Rust.

## Features

- **Memory Safe**: Written in Rust
- **Compact**: Optimized for minimal binary size and memory footprint
- **Fast**: Efficient implementations that outperform traditional utilities

## Installation

### Building from Source

```bash
git clone https://github.com/puppyutils/puppyutils
cd puppyutils
./build-release.sh
```

The project uses a pinned nightly toolchain specified in `rust-toolchain.toml` -
rustup will automatically install the correct version and components.

## Available Utilities

### Fully Implemented

- **`true`** - Exit with success status
- **`false`** - Exit with failure status
- **`uname`** - Display system information
- **`whoami`** - Display current username
- **`yes`** - Output strings repeatedly
- **`pwd`** - Print current working directory
- **`tty`** - Print terminal name
- **`wc`** - Count lines, words, characters, and bytes in files

### Partially Implemented

- **`touch`** - Update file timestamps and create files (supports -a, -m, -c flags)
- **`cat`** - Concatenate and display files (basic functionality implemented)
- **`mkdir`** - Create directories (basic functionality, missing -p, -m, -v options)
- **`ls`** - List directory contents (framework implemented but currently non-functional)

## Development Status

The project is in active development with a focus on correctness, performance,
and minimal binary size. Current priorities include completing core file system
utilities and implementing comprehensive option support for partially completed
commands.

## Usage

All utilities support standard `--help` and `--version` flags. Options after
`--` will be ignored and passed as values.

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for
guidelines on contributing to the project.

## License

This project is licensed under the [EUPL](https://eupl.eu/). For more
information, please see the [LICENSE](LICENSE) file.
