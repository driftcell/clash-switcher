use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Show Clash version
    Version,
    /// List all proxy groups
    Proxies,
    /// Show details of a specific proxy group (by number or name prefix)
    Proxy {
        /// Proxy group number or name/prefix
        name: String,
    },
    /// Show current proxy chain from GLOBAL selector
    Current,
    /// Switch proxy selection (interactive if no args provided)
    Switch {
        /// Proxy group name (e.g., GLOBAL)
        group: Option<String>,
        /// Target proxy name (e.g., DIRECT)
        proxy: Option<String>,
    },
    /// Set Clash mode (Global/Rule/Direct)
    Mode {
        /// Mode to set: global, rule, or direct (case-insensitive)
        mode: Option<String>,
    },
}
