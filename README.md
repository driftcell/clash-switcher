# Clash Switcher

A command-line tool for managing Clash proxy configurations via its RESTful API.

## Features

- View Clash version and current status
- List all proxy groups
- View detailed information about specific proxy groups
- Switch between proxy groups interactively or via command line
- Change Clash mode (Global/Rule/Direct)
- Display current proxy chain

## Installation

```bash
cargo build --release
```

The binary will be available at `target/release/clash-switcher`.

## Configuration

By default, the tool connects to Clash at `http://localhost:9090`. Make sure your Clash configuration has the external controller enabled:

```yaml
external-controller: 127.0.0.1:9090
# Optional: set a secret for API authentication
secret: your-secret-here
```

### Command Line Options

You can override the default API URL and provide a secret via command-line arguments:

```bash
# Use custom URL
clash-switcher --url http://192.168.1.100:9090 proxies

# Use custom URL with secret
clash-switcher --url http://192.168.1.100:9090 --secret your-secret proxies
```

These options are global and work with any command.

## Commands

### View Clash Version

```bash
clash-switcher version
```

### List All Proxy Groups

```bash
clash-switcher proxies
```

### View Proxy Group Details

By ID:
```bash
clash-switcher proxy 1
```

By name or prefix:
```bash
clash-switcher proxy GLOBAL
```

### View Current Status

```bash
clash-switcher current
```

Shows the current Clash mode and the complete proxy chain from GLOBAL to the active node.

### Switch Proxy

Interactive mode:
```bash
clash-switcher switch
```

Direct mode:
```bash
clash-switcher switch GLOBAL "proxy-name"
```

### Change Clash Mode

Interactive mode:
```bash
clash-switcher mode
```

Direct mode:
```bash
clash-switcher mode global
clash-switcher mode rule
clash-switcher mode direct
```

## Global Options

- `--url <URL>` - Clash API URL (default: `http://localhost:9090`)
- `--secret <SECRET>` - Clash API secret for authentication (default: empty)

Example with global options:
```bash
clash-switcher --url http://example.com:9090 --secret mytoken current
```

## Project Structure

```
src/
├── main.rs           # Entry point and command routing
├── cli.rs            # CLI argument definitions
├── client.rs         # Clash API client
├── models.rs         # Data structures
├── utils.rs          # Helper functions
└── commands/         # Command implementations
    ├── version.rs
    ├── proxies.rs
    ├── proxy.rs
    ├── current.rs
    ├── switch.rs
    └── mode.rs
```

## Dependencies

- clap - Command-line argument parsing
- reqwest - HTTP client
- serde - Serialization/deserialization
- colored - Terminal colors
- unicode-width - String width calculation for alignment

## Development

### Running Tests

This project uses [cargo-nextest](https://nexte.st/) for faster test execution.

Install nextest:
```bash
cargo install cargo-nextest
```

Run tests:
```bash
# Run all tests with nextest
cargo nextest run

# Run tests with standard cargo
cargo test

# Run specific test
cargo nextest run test_name

# Run with coverage
cargo install cargo-tarpaulin
cargo tarpaulin --out html
```

### Linting and Formatting

```bash
# Check code formatting
cargo fmt --check

# Format code
cargo fmt

# Run clippy lints
cargo clippy --all-targets --all-features
```

### Continuous Integration

The project uses GitHub Actions for CI/CD:

- **CI Workflow** - Runs on every push and pull request
  - Tests on Linux, macOS, and Windows
  - Code formatting checks
  - Clippy lints
  - Build verification
  - Code coverage

- **Release Workflow** - Triggered by version tags or manual dispatch
  - Builds binaries for multiple platforms
  - Creates GitHub releases with artifacts

## License

MIT