use anyhow::{Context as _, ensure};
use clap::{Parser, Subcommand};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Version {
    meta: bool,
    version: String,
}

struct ClashClient {
    base_url: String,
}

impl ClashClient {
    fn new(base_url: String) -> Self {
        Self { base_url }
    }

    fn version(&self) -> anyhow::Result<Version> {
        ensure!(!self.base_url.is_empty());
        reqwest::blocking::get(format!("{}/version", self.base_url))
            .context("Failed to get the clash version due to the request")?
            .json::<Version>()
            .context("Failed to parse clash version")
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Version,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let client = ClashClient::new("http://localhost:9090".into());

    match cli.command {
        Command::Version => {
            let version = client.version()?;
            println!(
                "Clash Version: {} - {}",
                version.version,
                if version.meta { "meta" } else { "non-meta" }
            );
        }
    }

    Ok(())
}
