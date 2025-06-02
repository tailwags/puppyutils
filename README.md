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

- **`true`** - Exit with success status
- **`false`** - Exit with failure status
- **`uname`** - Display system information
- **`whoami`** - Display current username
- **`yes`** - Output strings repeatedly
- **`pwd`** - Print current working directory

### In Development

- **`ls`** - List directory contents (basic functionality implemented)
- **`mkdir`** - Create directories (missing standard options like -p, -m, -v)

## Usage

All utilities support standard `--help` and `--version` flags. Options after
`--` will be ignored and passed as values.

### Project Structure

```
src/
├── bin/           # Individual utility implementations
│   ├── ls/        # Complex utilities may have subdirectories
│   ├── uname.rs
│   ├── whoami.rs
│   └── ...
├── lib.rs         # Shared library code and macros
└── main.rs        # Multi-call binary entry point
```

### Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for
guidelines on contributing to the project.

## License

This project is licensed under the [EUPL](https://eupl.eu/). For more
information, please see the [LICENSE](LICENSE) file.
