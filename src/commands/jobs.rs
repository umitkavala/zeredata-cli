use crate::api::ApiClient;
use crate::config::Config;
use crate::error::Result;
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::PathBuf;
use std::time::Duration;

pub async fn list() -> Result<()> {
    let config = Config::load()?;
    let client = ApiClient::from_config(&config)?;

    let jobs = client.list_jobs().await?;

    if jobs.is_empty() {
        println!("{}", style("No jobs found").yellow());
        return Ok(());
    }

    println!("\n{}", style(format!("Jobs ({})", jobs.len())).bold().underlined());
    println!();

    for job in jobs {
        let status_style = match job.status.as_str() {
            "completed" => style(&job.status).green(),
            "failed" => style(&job.status).red(),
            "running" => style(&job.status).cyan(),
            _ => style(&job.status).yellow(),
        };

        println!(
            "  {} {}",
            style(&job.job_id).cyan().bold(),
            style(&job.name).white()
        );

        let progress_str = if let Some(prog) = job.progress {
            format!("{}/{} scenes ({}%)", prog, job.num_scenes, (prog * 100) / job.num_scenes.max(1))
        } else {
            format!("{} scenes", job.num_scenes)
        };

        println!(
            "    {} {}  {} {}  {} {}",
            style("Status:").dim(),
            status_style,
            style("Progress:").dim(),
            progress_str,
            style("Created:").dim(),
            job.created_at
        );

        if let Some(completed) = job.completed_at {
            println!("    {} {}", style("Completed:").dim(), completed);
        }

        println!();
    }

    Ok(())
}

pub async fn status(job_id: String) -> Result<()> {
    let config = Config::load()?;
    let client = ApiClient::from_config(&config)?;

    let job = client.get_job(&job_id).await?;

    println!("\n{}", style("Job Details").bold().underlined());
    println!("  {} {}", style("ID:").dim(), job.job_id);
    println!("  {} {}", style("Name:").dim(), job.name);
    println!("  {} {}", style("Status:").dim(), job.status);
    println!("  {} {}", style("Scenes:").dim(), job.num_scenes);

    if let Some(progress) = job.progress {
        println!("  {} {}/{} ({}%)", style("Progress:").dim(), progress, job.num_scenes, (progress * 100) / job.num_scenes.max(1));
    }

    println!("  {} {}", style("Created:").dim(), job.created_at);

    if let Some(completed) = job.completed_at {
        println!("  {} {}", style("Completed:").dim(), completed);
    }

    println!();
    Ok(())
}

pub async fn create(
    name: String,
    num_scenes: i32,
    config_file: Option<PathBuf>,
) -> Result<()> {
    let config = Config::load()?;
    let client = ApiClient::from_config(&config)?;

    let config_yaml = if let Some(path) = config_file {
        Some(tokio::fs::read_to_string(path).await?)
    } else {
        None
    };

    println!("{} Creating job '{}'...", style("→").cyan(), style(&name).white().bold());

    let job = client.create_job(name, num_scenes, config_yaml).await?;

    println!(
        "{} Job created: {} ({})",
        style("✓").green().bold(),
        style(&job.name).white().bold(),
        style(&job.job_id).cyan()
    );
    println!("  {} {}", style("Status:").dim(), job.status);
    println!("  {} {}", style("Scenes:").dim(), job.num_scenes);

    Ok(())
}

pub async fn watch(job_id: String) -> Result<()> {
    let config = Config::load()?;
    let client = ApiClient::from_config(&config)?;

    println!("{} Watching job {}...\n", style("→").cyan(), style(&job_id).cyan().bold());

    let pb = ProgressBar::new(100);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {percent}% {msg}")
            .unwrap()
            .progress_chars("#>-"),
    );

    loop {
        match client.get_job_progress(&job_id).await {
            Ok(progress) => {
                pb.set_position(progress.progress_percent as u64);

                let msg = if let Some(eta) = progress.eta_seconds {
                    format!("{}/{} scenes - ETA: {}s", progress.scenes_generated, progress.progress, eta)
                } else {
                    format!("{}/{} scenes", progress.scenes_generated, progress.progress)
                };
                pb.set_message(msg);

                if progress.status == "completed" {
                    pb.finish_with_message(style("Completed!").green().to_string());
                    println!("\n{} Job completed successfully", style("✓").green().bold());
                    break;
                } else if progress.status == "failed" {
                    pb.finish_with_message(style("Failed").red().to_string());
                    println!("\n{} Job failed", style("✗").red().bold());
                    break;
                }
            }
            Err(e) => {
                pb.finish_with_message(style("Error").red().to_string());
                return Err(e);
            }
        }

        tokio::time::sleep(Duration::from_secs(2)).await;
    }

    Ok(())
}

pub async fn cancel(job_id: String, force: bool) -> Result<()> {
    let config = Config::load()?;
    let client = ApiClient::from_config(&config)?;

    if !force {
        let confirm = inquire::Confirm::new(&format!("Cancel job '{}'?", job_id))
            .with_default(false)
            .prompt()
            .unwrap_or(false);

        if !confirm {
            println!("{}", style("Cancelled").yellow());
            return Ok(());
        }
    }

    client.cancel_job(&job_id).await?;

    println!(
        "{} Job cancelled: {}",
        style("✓").green().bold(),
        style(&job_id).cyan()
    );

    Ok(())
}

/// Quick Start Mode - Generate with procedural objects (no assets needed)
pub async fn quick_start(
    num_scenes: i32,
    objects_range: Option<String>,
    environment: Option<String>,
) -> Result<()> {
    let config = Config::load()?;
    let client = ApiClient::from_config(&config)?;

    // Parse object range (e.g., "20-30")
    let (min_objects, max_objects) = if let Some(range) = objects_range {
        let parts: Vec<&str> = range.split('-').collect();
        if parts.len() != 2 {
            return Err(crate::error::CliError::Config(
                "Object range must be in format MIN-MAX (e.g., 20-30)".to_string()
            ));
        }
        let min = parts[0].parse::<i32>().map_err(|_| {
            crate::error::CliError::Config("Invalid min objects number".to_string())
        })?;
        let max = parts[1].parse::<i32>().map_err(|_| {
            crate::error::CliError::Config("Invalid max objects number".to_string())
        })?;
        (min, max)
    } else {
        (25, 35) // Default from benchmark configs
    };

    let env_type = environment.unwrap_or_else(|| "warehouse_shelf".to_string());

    println!("{} Quick Start Mode", style("→").cyan());
    println!("  {} {}", style("Scenes:").dim(), num_scenes);
    println!("  {} {}-{}", style("Objects:").dim(), min_objects, max_objects);
    println!("  {} {}", style("Environment:").dim(), env_type);
    println!();

    // Generate procedural config YAML
    let config_yaml = generate_procedural_config(num_scenes, min_objects, max_objects, &env_type);

    println!("{} Creating quick start job...", style("→").cyan());

    let job_name = format!("Quick Start - {} scenes", num_scenes);
    let job = client.create_job(job_name.clone(), num_scenes, Some(config_yaml)).await?;

    println!(
        "{} Job created: {} ({})",
        style("✓").green().bold(),
        style(&job.name).white().bold(),
        style(&job.job_id).cyan()
    );
    println!("  {} {}", style("Status:").dim(), job.status);
    println!("  {} {} scenes with {}-{} procedural objects each",
        style("Config:").dim(),
        num_scenes,
        min_objects,
        max_objects
    );
    println!();
    println!("{} Track progress with: {} {}",
        style("→").cyan(),
        style("zere jobs watch").white().bold(),
        style(&job.job_id).cyan()
    );

    Ok(())
}

/// Generate procedural config YAML for Quick Start mode
fn generate_procedural_config(num_scenes: i32, min_objects: i32, max_objects: i32, environment: &str) -> String {
    format!(
r#"num_scenes: {}
scenes_per_batch: {}
headless: true
max_workers: 1

scene:
  objects:
    object_categories:
      - box_small
      - box_medium
      - bottle
      - pouch
    num_objects_range: [{}, {}]
    randomize_rotation: true
    use_physics_settling: true
    physics_settling_steps: 150

  environment:
    environment_type: {}
    bin_size: [0.64, 0.44, 0.3]

camera:
  position_range_x: [0.5, 0.8]
  position_range_y: [0.6, 0.8]
  position_range_z: [0.6, 1.0]
  look_at_target: [0, 0.15, 0]

  intrinsics:
    focal_length: 35
    sensor_width: 36
    resolution: [1280, 720]

lighting:
  profile: StandardOverhead
  num_lights_range: [4, 6]
  intensity_range: [2.0, 4.0]
"#,
        num_scenes,
        num_scenes.min(100), // Batch size
        min_objects,
        max_objects,
        environment
    )
}
