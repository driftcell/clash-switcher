use crate::client::ClashClient;
use anyhow::Result;
use colored::Colorize;

pub fn execute(client: &ClashClient) -> Result<()> {
    let config = client.get_config()?;
    let proxies = client.get_proxies()?;

    // Display current Clash mode
    println!("\n{}", "Current Status:".bright_yellow().bold());
    println!();
    println!(
        "{} {}",
        "Clash Mode:".bright_yellow(),
        config.mode.bright_green().bold()
    );

    let mode_desc = match config.mode.to_lowercase().as_str() {
        "global" => "All traffic goes through GLOBAL proxy",
        "rule" => "Traffic routing based on rules",
        "direct" => "All traffic bypasses proxy",
        _ => "Unknown mode",
    };
    println!(
        "{} {}",
        "           ".bright_yellow(),
        mode_desc.bright_black()
    );
    println!();

    // Start from GLOBAL selector
    if let Some(global) = proxies.proxies.get("GLOBAL") {
        println!("{}", "GLOBAL Proxy Chain:".bright_yellow().bold());
        println!();

        let mut chain = vec!["GLOBAL".to_string()];
        let mut current_proxy = global;

        // Follow the chain until we reach a non-selector node
        while !current_proxy.now.is_empty() {
            let next_name = &current_proxy.now;
            chain.push(next_name.clone());

            // Try to get the next proxy in chain
            if let Some(next_proxy) = proxies.proxies.get(next_name) {
                // Check if it's a selector or the final node
                if next_proxy.proxy_type == "Selector" || next_proxy.proxy_type == "URLTest" {
                    current_proxy = next_proxy;
                } else {
                    // Reached final node
                    break;
                }
            } else {
                break;
            }
        }

        // Display the chain
        for (i, name) in chain.iter().enumerate() {
            if i == 0 {
                println!("  {} {}", "┌─".bright_blue(), name.bright_cyan());
            } else if i == chain.len() - 1 {
                println!(
                    "  {} {} {}",
                    "└─→".bright_green(),
                    name.bright_green().bold(),
                    "(active)".bright_black()
                );
            } else {
                println!("  {} {}", "├─→".bright_blue(), name.bright_cyan());
            }
        }

        // Show the type of final node
        if let Some(final_proxy) = proxies.proxies.get(&chain[chain.len() - 1]) {
            println!();
            println!("  {} {}", "Type:".bright_yellow(), final_proxy.proxy_type);
        }
    } else {
        println!("{}", "GLOBAL selector not found".red());
    }

    Ok(())
}
