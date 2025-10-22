use crate::client::ClashClient;
use anyhow::Result;

pub fn execute(client: &ClashClient) -> Result<()> {
    let version = client.version()?;
    println!(
        "Clash Version: {} - {}",
        version.version,
        if version.meta { "meta" } else { "non-meta" }
    );
    Ok(())
}
