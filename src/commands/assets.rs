use crate::api::ApiClient;
use crate::config::Config;
use crate::error::Result;
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::PathBuf;

pub async fn list() -> Result<()> {
    let config = Config::load()?;
    let client = ApiClient::from_config(&config)?;

    let assets = client.list_assets().await?;

    if assets.is_empty() {
        println!("{}", style("No assets found").yellow());
        return Ok(());
    }

    println!("\n{}", style(format!("Assets ({})", assets.len())).bold().underlined());
    println!();

    for asset in assets {
        let size_mb = asset.size_bytes as f64 / 1_048_576.0;
        println!(
            "  {} {}",
            style(&asset.asset_id).cyan().bold(),
            style(&asset.name).white()
        );
        println!(
            "    {} {}  {} {:.2} MB  {} {}",
            style("Type:").dim(),
            asset.file_type,
            style("Size:").dim(),
            size_mb,
            style("Created:").dim(),
            asset.created_at
        );
        println!();
    }

    Ok(())
}

pub async fn info(asset_id: String) -> Result<()> {
    let config = Config::load()?;
    let client = ApiClient::from_config(&config)?;

    let asset = client.get_asset(&asset_id).await?;

    println!("\n{}", style("Asset Details").bold().underlined());
    println!("  {} {}", style("ID:").dim(), asset.asset_id);
    println!("  {} {}", style("Name:").dim(), asset.name);
    println!("  {} {}", style("Type:").dim(), asset.file_type);
    println!("  {} {:.2} MB", style("Size:").dim(), asset.size_bytes as f64 / 1_048_576.0);
    println!("  {} {}", style("Created:").dim(), asset.created_at);

    if let Some(thumbnail) = asset.thumbnail_url {
        println!("  {} {}", style("Thumbnail:").dim(), thumbnail);
    }

    println!();
    Ok(())
}

pub async fn upload(
    file_path: PathBuf,
    name: Option<String>,
    category: Option<String>,
    tags: Vec<String>,
) -> Result<()> {
    let config = Config::load()?;
    let client = ApiClient::from_config(&config)?;

    if !file_path.exists() {
        return Err(crate::error::CliError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("File not found: {}", file_path.display()),
        )));
    }

    let file_name = file_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("upload")
        .to_string();

    let asset_name = name.unwrap_or_else(|| file_name.clone());

    println!(
        "{} {}",
        style("Uploading").cyan(),
        style(file_path.display()).white()
    );

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    pb.set_message("Uploading...");
    pb.enable_steady_tick(std::time::Duration::from_millis(100));

    let asset = client
        .upload_asset(&file_path, asset_name.clone(), category, tags)
        .await?;

    pb.finish_and_clear();

    println!(
        "{} Asset uploaded: {} ({})",
        style("✓").green().bold(),
        style(&asset.name).white().bold(),
        style(&asset.asset_id).cyan()
    );

    Ok(())
}

pub async fn delete(asset_id: String, force: bool) -> Result<()> {
    let config = Config::load()?;
    let client = ApiClient::from_config(&config)?;

    // Get asset info first
    let asset = client.get_asset(&asset_id).await?;

    if !force {
        let confirm = inquire::Confirm::new(&format!(
            "Delete asset '{}' ({})?",
            asset.name, asset.asset_id
        ))
        .with_default(false)
        .prompt()
        .unwrap_or(false);

        if !confirm {
            println!("{}", style("Cancelled").yellow());
            return Ok(());
        }
    }

    client.delete_asset(&asset_id).await?;

    println!(
        "{} Asset deleted: {}",
        style("✓").green().bold(),
        style(&asset.name).white()
    );

    Ok(())
}
