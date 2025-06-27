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
- **`tty`** - Print terminal name

### Currently in Development

- **`ls`** - List directory contents (basic functionality implemented)
- **`cat`** - Concatenate and display files (basic functionality implemented,
  missing most options)
- **`mkdir`** - Create directories (missing standard options like -p, -m, -v)

### Missing

The project is still in early development, so the majority of standard Unix
utilities remain unimplemented. Rather than maintaining an exhaustive list of
missing commands, we're focusing on building a solid foundation with the core
utilities first. As the project matures and more commands are added, this
section will be expanded to highlight specific gaps and implementation
priorities.

## Usage

All utilities support standard `--help` and `--version` flags. Options after
`--` will be ignored and passed as values.

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for
guidelines on contributing to the project.

## License

This project is licensed under the [EUPL](https://eupl.eu/). For more
information, please see the [LICENSE](LICENSE) file.
