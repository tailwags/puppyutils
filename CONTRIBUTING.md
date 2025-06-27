# Contributing to Puppyutils

We welcome contributions to Puppyutils. This guide will help you understand the project and contribute effectively.

## Code of Conduct

This project follows the organization code of conduct. An up-to-date version can be found in the [here](https://github.com/puppyutils/.github/blob/main/CODE_OF_CONDUCT.md).

## Project Overview

Puppyutils is a Rust implementation of core Unix utilities focused on four key principles:

- **Memory Safety**: Written in Rust with zero tolerance for memory-related bugs
- **Minimal Size**: Aggressive optimization for the smallest possible binary footprint
- **Correctness**: Faithful implementations that match standard Unix behavior
- **Clean Organization**: Maintainable codebase with consistent patterns and clear structure

The project uses a multi-call binary architecture where a single executable acts as different utilities based on how it's invoked (similar to BusyBox). This approach significantly reduces disk space and memory usage compared to separate binaries.

### Current Status

The project is in active development with implementations ranging from simple utilities to more complex ones with partial feature sets and ongoing development.

## Getting Started

### Prerequisites

- Rust nightly toolchain (automatically managed via `rust-toolchain.toml`)
- Git

### Building

```bash
git clone https://github.com/puppyutils/puppyutils
cd puppyutils
cargo build
```

For optimized release builds: `./build-release.sh`

This uses nightly features and build-std for maximum size optimization.

## Project Structure

```
puppyutils/
├── src/
│   ├── main.rs              # Multi-call binary dispatcher
│   ├── lib.rs               # Shared library with macros and utilities
│   └── bin/                 # Individual utility implementations
│       ├── simple.rs        # Single-file utilities
│       └── complex/         # Multi-file utilities
│           ├── main.rs
│           ├── options.rs
│           └── settings.rs
├── docs/                    # Help text for each utility
│   ├── ls.txt
│   ├── cat.txt
│   └── ...
├── build-release.sh         # Optimized build script
├── rust-toolchain.toml      # Pinned nightly toolchain
└── CONTRIBUTING.md          # This file
```

### Key Components

- **`src/main.rs`**: Entry point that dispatches to appropriate utility based on binary name
- **`src/lib.rs`**: Shared code including the `cli!` macro, error types, and utility functions
- **`src/bin/`**: Individual utility implementations following consistent patterns
- **`docs/`**: Help text files included at compile time via the `help_text!` macro

## Adding New Utilities

### 1. Implementation Structure

**Simple utilities** (single file):

```
src/bin/utilname.rs
```

**Complex utilities** (multiple files):

```
src/bin/utilname/
├── main.rs
├── options.rs
└── settings.rs
```

### 2. Implementation Pattern

Every utility follows this template:

```rust
use std::io::{Write, stdout};
use puppyutils::{Result, cli};

pub fn main() -> Result {
    let mut stdout = stdout();

    cli! {
        "utilname", stdout, #error
        Short('a') | Long("all") => {
            // Handle --all flag
        }
        Value(path) => {
            // Handle positional arguments
        }
    };

    // Implementation logic here
    // Use rustix for system calls when possible
    // Write directly to stdout/stderr
    // Flush buffers explicitly

    Ok(())
}
```

### 3. Registration

Add your utility to `src/main.rs`:

```rust
// In the mod bin block:
pub mod utilname;

// In the match statement:
b"utilname" => bin::utilname::main(),
```

### 4. Documentation

Create `docs/utilname.txt` with help text that will be shown for `--help`:

```
Usage: utilname [OPTION]... [FILE]...
Brief description of what the utility does.

Options:
  -a, --all          include entries starting with .
      --help         display this help and exit
      --version      output version information and exit
```

## Coding Standards

### Core Principles

- **Minimal comments**: Code should be self-documenting (exception: complex system-level code like `get_umask()`)
- **Use `rustix`**: Prefer `rustix` over `std` for system calls when possible
- **Direct I/O**: Write directly to stdout/stderr using `Write` trait methods
- **Explicit flushing**: Always flush output buffers before returning
- **Consistent errors**: Return `puppyutils::Result<T, Exit>` for error handling

### Performance Requirements

- **Minimize allocations**: Use zero-copy approaches, stack allocation, and streaming where possible
- **Binary size matters**: Avoid unnecessary dependencies, use `#[inline]` judiciously
- **Memory efficiency**: Consider the memory footprint of data structures

### Code Patterns

- Use the `cli!` macro for argument parsing (handles `--help` and `--version` automatically)
- Use `puppyutils::Exit` enum for error types with automatic conversions
- Follow existing patterns in `src/bin/` for consistency
- Use `bitflags!` for option flags when appropriate
- Include help text via `help_text!` macro

## Development Workflow

### Local Development

1. **Format code**: `cargo fmt`
2. **Lint code**: `cargo clippy --all-targets --all-features`
3. **Check formatting**: `cargo fmt --check`
4. **Build**: `cargo build`
5. **Release build**: `./build-release.sh`

### Testing

Currently, the project has minimal testing infrastructure. When implementing utilities:

- Test manually with various inputs and edge cases
- Verify compatibility with standard Unix utilities
- Check error handling and exit codes
- Test `--help` and `--version` flags

## Submission Process

### 1. Preparation

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/add-utilname`
3. Implement your changes following the coding standards
4. Test thoroughly with manual testing

### 2. Code Quality

Before submitting:

```bash
# Format code
cargo fmt

# Run linting
cargo clippy --all-targets --all-features

# Ensure it builds
cargo build

# Test release build
./build-release.sh
```

### 3. Pull Request

Submit a PR with:

- **Clear title**: "Add utilname utility" or "Fix bug in ls option parsing"
- **Detailed description**: What the change does, why it's needed, how it was tested
- **Reference issues**: Link to any related issues
- **Breaking changes**: Note any compatibility concerns

### 4. Review Process

PRs will be reviewed for:

- **Correctness**: Does it implement the utility correctly?
- **Performance**: Does it follow size and speed optimization principles?
- **Code style**: Does it match existing patterns and standards?
- **Completeness**: Are all standard options implemented or documented as missing?

### 5. CI Requirements

All PRs must pass:

- **Format check**: `cargo fmt --check`
- **Lint check**: `cargo clippy --all-targets --all-features`
- **Build check**: Both debug and release builds must succeed
- **Binary size**: Release binary size should not increase unnecessarily

## Getting Help

- **Questions**: Open an issue for questions about contributing or implementation details
- **Discussions**: Use GitHub Discussions for broader architectural questions
- **Issues**: Report bugs or request features via GitHub Issues

## Priority Areas

Current focus areas for contributions:

1. **Complete existing utilities**: Add missing options to partially implemented utilities
2. **New core utilities**: Implement essential utilities following established patterns
3. **Performance optimization**: Reduce allocations, improve algorithms
4. **Testing infrastructure**: Set up proper testing framework
5. **Documentation**: Improve help text and edge case handling

The goal is building a solid foundation with the most essential utilities before expanding to the full coreutils suite.
