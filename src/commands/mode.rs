use crate::client::ClashClient;
use anyhow::{Context, Result};
use colored::Colorize;
use std::io::{self, Write};

pub fn execute(client: &ClashClient, mode: Option<String>) -> Result<()> {
    // Get current config
    let config = client.get_config()?;

    let new_mode = if let Some(m) = mode {
        // Validate and normalize mode
        let normalized = m.to_lowercase();
        match normalized.as_str() {
            "global" => "Global",
            "rule" => "Rule",
            "direct" => "Direct",
            _ => {
                anyhow::bail!(
                    "Invalid mode '{}'. Valid modes are: global, rule, direct",
                    m
                );
            }
        }
    } else {
        // Interactive mode: show current and let user choose
        println!("\n{}", "Clash Mode".bright_yellow().bold());
        println!();
        println!(
            "{} {}",
            "Current mode:".bright_yellow(),
            config.mode.bright_green().bold()
        );
        println!();
        println!("Available modes:");
        println!(
            "  {} {} - All traffic goes through proxy",
            "[1]".bright_cyan(),
            "Global".bold()
        );
        println!(
            "  {} {} - Traffic routing based on rules",
            "[2]".bright_cyan(),
            "Rule".bold()
        );
        println!(
            "  {} {} - All traffic goes direct (no proxy)",
            "[3]".bright_cyan(),
            "Direct".bold()
        );
        println!();

        print!("{} ", "Enter mode number:".bright_yellow());
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let mode_idx: usize = input.trim().parse().context("Invalid number")?;

        match mode_idx {
            1 => "Global",
            2 => "Rule",
            3 => "Direct",
            _ => anyhow::bail!("Invalid mode number"),
        }
    };

    // Check if already in this mode
    if config.mode == new_mode {
        println!(
            "{} Already in {} mode",
            "ℹ".bright_blue(),
            new_mode.bright_cyan()
        );
        return Ok(());
    }

    // Set the new mode
    client.set_mode(new_mode)?;

    println!();
    println!(
        "{} Mode changed: {} → {}",
        "✓".bright_green().bold(),
        config.mode.bright_yellow(),
        new_mode.bright_green().bold()
    );

    // Show what this means
    println!();
    match new_mode {
        "Global" => println!(
            "  {} All traffic will now go through the GLOBAL proxy",
            "→".bright_blue()
        ),
        "Rule" => println!(
            "  {} Traffic will be routed based on your rules",
            "→".bright_blue()
        ),
        "Direct" => println!("  {} All traffic will bypass the proxy", "→".bright_blue()),
        _ => {}
    }

    Ok(())
}
