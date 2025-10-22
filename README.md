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
```

## Usage

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

## License

MIT