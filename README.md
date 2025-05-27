# Puppyutils

An efficient and correct implementation of coreutils, util-linux and other core
Linux system utilities in Rust.

## Features

- **Compact**: Optimized for minimal binary size and memory footprint
- **Memory Safe**: Built with Rust to prevent common security vulnerabilities
- **Fast**: Efficient implementations that outperform traditional utilities

<!-- - **Drop-in Replacement**: 99% compatible with existing scripts and workflows -->

## Installation

### Building from Source

```bash
git clone https://github.com/puppyutils/puppyutils
cd puppyutils
./build-release.sh
```

## Available Utilities

- **`true`** - Exit with success status
- **`false`** - Exit with failure status
- **`uname`** - Display system information
- **`whoami`** - Display current username
- **`yes`** - Output strings repeatedly
- **`ls`** - List directory contents (in development)
- **`mkdir`** - Create directories (in development)

All utilities support `--help` and `--version` flags.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on contributing to the
project.

## License

TBA
