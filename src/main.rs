mod cli;
mod client;
mod commands;
mod models;
mod utils;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Command};
use client::ClashClient;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let client = ClashClient::new("http://localhost:9090".into());

    match cli.command {
        Command::Version => commands::version::execute(&client),
        Command::Proxies => commands::proxies::execute(&client),
        Command::Proxy { name } => commands::proxy::execute(&client, &name),
        Command::Current => commands::current::execute(&client),
        Command::Switch { group, proxy } => commands::switch::execute(&client, group, proxy),
        Command::Mode { mode } => commands::mode::execute(&client, mode),
    }
}
