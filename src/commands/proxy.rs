use crate::client::ClashClient;
use anyhow::Result;
use colored::Colorize;

pub fn execute(client: &ClashClient, name: &str) -> Result<()> {
    // Try to parse as number first
    let proxy_name = if let Ok(idx) = name.parse::<usize>() {
        // Get proxy by index
        let proxies = client.get_proxies()?;
        let mut groups: Vec<_> = proxies
            .proxies
            .iter()
            .filter(|(_, info)| info.proxy_type == "Selector" || info.proxy_type == "URLTest")
            .collect();
        groups.sort_by_key(|(name, _)| *name);

        if idx == 0 || idx > groups.len() {
            anyhow::bail!("Invalid proxy group number. Use 'proxies' to see available groups.");
        }
        groups[idx - 1].0.to_string()
    } else {
        // Try to match by prefix
        let proxies = client.get_proxies()?;
        let matches: Vec<_> = proxies
            .proxies
            .iter()
            .filter(|(n, _)| {
                n.starts_with(name) || n.to_lowercase().starts_with(&name.to_lowercase())
            })
            .collect();

        if matches.is_empty() {
            anyhow::bail!(
                "No proxy group found matching '{}'. Use 'proxies' to see available groups.",
                name
            );
        } else if matches.len() > 1 {
            println!("Multiple matches found:");
            for (n, _) in matches {
                println!("  - {}", n);
            }
            anyhow::bail!("Please be more specific or use the ID number.");
        } else {
            matches[0].0.to_string()
        }
    };

    let proxy = client.get_proxy(&proxy_name)?;
    println!(
        "\n{} {}",
        "Proxy:".bright_yellow(),
        proxy_name.bright_cyan()
    );
    println!("{} {}", "Type:".bright_yellow(), proxy.proxy_type);

    if !proxy.now.is_empty() {
        println!(
            "{} {}",
            "Current:".bright_yellow(),
            proxy.now.bright_green()
        );
    }

    if !proxy.all.is_empty() {
        println!(
            "\n{} ({}):",
            "Available proxies".bright_yellow(),
            proxy.all.len()
        );
        for (i, node) in proxy.all.iter().enumerate() {
            let marker = if !proxy.now.is_empty() && node == &proxy.now {
                "●".bright_green()
            } else {
                "○".normal()
            };
            println!("  {} {}. {}", marker, i + 1, node);
        }
    }

    if !proxy.history.is_empty() {
        println!("\n{}:", "Latency history".bright_yellow());
        for item in &proxy.history {
            println!("  {} - {}ms", item.time, item.delay);
        }
    }

    Ok(())
}
