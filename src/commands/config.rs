use crate::config::Config;
use crate::error::Result;
use console::style;

pub async fn set_endpoint(endpoint: String) -> Result<()> {
    let mut config = Config::load()?;
    config.set_endpoint(endpoint.clone());
    config.save()?;

    println!(
        "{} API endpoint set to: {}",
        style("✓").green().bold(),
        style(&endpoint).cyan()
    );

    Ok(())
}

pub async fn show() -> Result<()> {
    let config = Config::load()?;

    println!("\n{}", style("Configuration").bold().underlined());
    println!("  {} {}", style("API Endpoint:").dim(), config.api.endpoint);
    println!(
        "  {} {}",
        style("Authenticated:").dim(),
        if config.is_authenticated() {
            style("Yes").green()
        } else {
            style("No").red()
        }
    );

    if let Ok(path) = Config::config_path() {
        println!("  {} {}", style("Config File:").dim(), path.display());
    }

    println!();
    Ok(())
}
