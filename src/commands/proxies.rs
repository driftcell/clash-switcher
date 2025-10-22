use crate::client::ClashClient;
use crate::utils::pad_string;
use anyhow::Result;
use colored::Colorize;

pub fn execute(client: &ClashClient) -> Result<()> {
    let proxies = client.get_proxies()?;

    // Filter and display only Selector and URLTest types (proxy groups)
    let mut groups: Vec<_> = proxies
        .proxies
        .iter()
        .filter(|(_, info)| info.proxy_type == "Selector" || info.proxy_type == "URLTest")
        .collect();

    groups.sort_by_key(|(name, _)| *name);

    println!("Proxy Groups:\n");
    let id_width = 5;
    let name_width = 30;
    let type_width = 12;

    println!(
        "{} {} {} CURRENT",
        pad_string("ID", id_width),
        pad_string("NAME", name_width),
        pad_string("TYPE", type_width)
    );
    println!("{}", "-".repeat(85));

    for (idx, (name, info)) in groups.iter().enumerate() {
        let current = if !info.now.is_empty() {
            &info.now
        } else {
            "N/A"
        };

        let id_str = format!("[{}]", idx + 1);
        println!(
            "{} {} {} {}",
            pad_string(&id_str.bright_cyan().to_string(), id_width + 9), // +9 for ANSI color codes
            pad_string(name, name_width),
            pad_string(&info.proxy_type, type_width),
            current
        );
    }

    println!("\n{}", "Usage:".bright_yellow());
    println!(
        "  {} {} {}    View details by ID",
        "proxy".green(),
        "1".bright_cyan(),
        " ".repeat(7)
    );
    println!(
        "  {} {}  View details by name prefix",
        "proxy".green(),
        "GLOBAL".bright_cyan()
    );

    Ok(())
}
