use crate::client::ClashClient;
use crate::utils::pad_string;
use anyhow::{Context, Result};
use colored::Colorize;
use std::io::{self, Write};

pub fn execute(client: &ClashClient, group: Option<String>, proxy: Option<String>) -> Result<()> {
    let proxies = client.get_proxies()?;

    // Get all selector groups
    let mut groups: Vec<_> = proxies
        .proxies
        .iter()
        .filter(|(_, info)| info.proxy_type == "Selector")
        .collect();
    groups.sort_by_key(|(name, _)| *name);

    if groups.is_empty() {
        anyhow::bail!("No selector proxy groups found");
    }

    // Determine the group to switch
    let selected_group = if let Some(g) = group {
        g
    } else {
        // Interactive mode: select group
        println!("\n{}", "Select Proxy Group:".bright_yellow().bold());
        println!();

        for (idx, (name, info)) in groups.iter().enumerate() {
            let current = if !info.now.is_empty() {
                format!("→ {}", info.now.bright_green())
            } else {
                "".to_string()
            };
            println!(
                "  {} {} {}",
                format!("[{}]", idx + 1).bright_cyan(),
                pad_string(name, 25),
                current
            );
        }

        println!();
        print!("{} ", "Enter group number:".bright_yellow());
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let group_idx: usize = input.trim().parse().context("Invalid number")?;

        if group_idx == 0 || group_idx > groups.len() {
            anyhow::bail!("Invalid group number");
        }

        groups[group_idx - 1].0.to_string()
    };

    // Get the selected group's info
    let proxy_info = client
        .get_proxy(&selected_group)
        .context("Failed to get proxy group info")?;

    // Check if it's a selector
    if proxy_info.proxy_type != "Selector" {
        anyhow::bail!(
            "Cannot switch '{}': not a Selector (type: {})",
            selected_group,
            proxy_info.proxy_type
        );
    }

    // Determine the proxy to switch to
    let selected_proxy = if let Some(p) = proxy {
        // Check if the target proxy is in the available list
        if !proxy_info.all.contains(&p) {
            println!(
                "{} '{}' is not available in group '{}'",
                "Error:".red().bold(),
                p,
                selected_group
            );
            println!("\n{}:", "Available proxies".bright_yellow());
            for (i, name) in proxy_info.all.iter().enumerate() {
                let marker = if name == &proxy_info.now {
                    "●".bright_green()
                } else {
                    "○".normal()
                };
                println!("  {} {}. {}", marker, i + 1, name);
            }
            anyhow::bail!("Invalid proxy selection");
        }
        p
    } else {
        // Interactive mode: select proxy
        println!();
        println!(
            "{} {}",
            "Select Proxy for".bright_yellow().bold(),
            selected_group.bright_cyan().bold()
        );
        println!();

        for (idx, name) in proxy_info.all.iter().enumerate() {
            let marker = if name == &proxy_info.now {
                "●".bright_green()
            } else {
                "○".normal()
            };
            println!(
                "  {} {} {}",
                marker,
                format!("[{}]", idx + 1).bright_cyan(),
                name
            );
        }

        println!();
        print!("{} ", "Enter proxy number:".bright_yellow());
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let proxy_idx: usize = input.trim().parse().context("Invalid number")?;

        if proxy_idx == 0 || proxy_idx > proxy_info.all.len() {
            anyhow::bail!("Invalid proxy number");
        }

        proxy_info.all[proxy_idx - 1].clone()
    };

    // Perform the switch
    client.switch_proxy(&selected_group, &selected_proxy)?;

    println!();
    println!(
        "{} Switched '{}' to '{}'",
        "✓".bright_green().bold(),
        selected_group.bright_cyan(),
        selected_proxy.bright_green()
    );

    // Show the new chain if switching GLOBAL
    if selected_group == "GLOBAL" {
        println!();
        let proxies = client.get_proxies()?;
        if let Some(global) = proxies.proxies.get("GLOBAL") {
            println!("{}", "New proxy chain:".bright_yellow());
            let mut chain = vec!["GLOBAL".to_string()];
            let mut current_proxy = global;

            while !current_proxy.now.is_empty() {
                let next_name = &current_proxy.now;
                chain.push(next_name.clone());

                if let Some(next_proxy) = proxies.proxies.get(next_name) {
                    if next_proxy.proxy_type == "Selector" || next_proxy.proxy_type == "URLTest" {
                        current_proxy = next_proxy;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            for (i, name) in chain.iter().enumerate() {
                if i == 0 {
                    println!("  {} {}", "┌─".bright_blue(), name.bright_cyan());
                } else if i == chain.len() - 1 {
                    println!("  {} {}", "└─→".bright_green(), name.bright_green().bold());
                } else {
                    println!("  {} {}", "├─→".bright_blue(), name.bright_cyan());
                }
            }
        }
    }

    Ok(())
}
