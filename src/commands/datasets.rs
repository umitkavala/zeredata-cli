use crate::api::ApiClient;
use crate::config::Config;
use crate::error::Result;
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::PathBuf;

pub async fn list() -> Result<()> {
    let config = Config::load()?;
    let client = ApiClient::from_config(&config)?;

    let datasets = client.list_datasets().await?;

    if datasets.is_empty() {
        println!("{}", style("No datasets found").yellow());
        return Ok(());
    }

    println!("\n{}", style(format!("Datasets ({})", datasets.len())).bold().underlined());
    println!();

    for dataset in datasets {
        let size_gb = dataset.size_bytes as f64 / 1_073_741_824.0;
        println!(
            "  {} {}",
            style(&dataset.job_id).cyan().bold(),
            style(&dataset.name).white()
        );
        println!(
            "    {} {}  {} {:.2} GB  {} {}",
            style("Format:").dim(),
            dataset.format,
            style("Size:").dim(),
            size_gb,
            style("Created:").dim(),
            dataset.created_at
        );
        println!();
    }

    Ok(())
}

pub async fn info(dataset_id: String) -> Result<()> {
    let config = Config::load()?;
    let client = ApiClient::from_config(&config)?;

    let dataset = client.get_dataset(&dataset_id).await?;

    println!("\n{}", style("Dataset Details").bold().underlined());
    println!("  {} {}", style("Job ID:").dim(), dataset.job_id);
    println!("  {} {}", style("Name:").dim(), dataset.name);
    println!("  {} {}", style("Format:").dim(), dataset.format);
    println!("  {} {:.2} GB", style("Size:").dim(), dataset.size_bytes as f64 / 1_073_741_824.0);
    println!("  {} {}", style("Created:").dim(), dataset.created_at);
    println!();

    Ok(())
}

pub async fn download(job_id: String, output_path: Option<PathBuf>) -> Result<()> {
    let config = Config::load()?;
    let client = ApiClient::from_config(&config)?;

    println!("{} Generating download URL...", style("→").cyan());

    let download_info = client.get_download_url(&job_id).await?;
    let size_mb = download_info.size_bytes as f64 / 1_048_576.0;

    println!(
        "{} Size: {:.2} MB, Format: {}, Expires: {}",
        style("ℹ").blue(),
        size_mb,
        download_info.format,
        download_info.expires_at
    );

    let output = output_path.unwrap_or_else(|| {
        PathBuf::from(format!("{}.zip", job_id))
    });

    println!("{} Downloading to {}...", style("→").cyan(), output.display());

    let pb = ProgressBar::new(download_info.size_bytes as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .progress_chars("#>-"),
    );

    // Download (progress tracking would require streaming implementation)
    client.download_dataset(&download_info.download_url, &output).await?;

    pb.finish_with_message(style("Downloaded!").green().to_string());

    println!(
        "\n{} Dataset downloaded: {}",
        style("✓").green().bold(),
        style(output.display()).white()
    );

    Ok(())
}
