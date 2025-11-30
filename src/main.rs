mod api;
mod commands;
mod config;
mod error;
mod tui;

use clap::{Parser, Subcommand};
use console::style;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "zere")]
#[command(about = "Zere CLI - Synthetic data generation for robotics", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Launch interactive TUI mode
    #[arg(short, long)]
    interactive: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Authentication commands
    #[command(subcommand)]
    Auth(AuthCommands),

    /// Login to Zere platform
    Login {
        /// Email address
        #[arg(long)]
        email: Option<String>,

        /// Password
        #[arg(long)]
        password: Option<String>,

        /// API key (alternative to email/password)
        #[arg(long)]
        api_key: Option<String>,
    },

    /// Logout from Zere platform
    Logout,

    /// Show current user information
    Whoami,

    /// Asset management commands
    #[command(subcommand)]
    Assets(AssetCommands),

    /// Job management commands
    #[command(subcommand)]
    Jobs(JobCommands),

    /// Dataset management commands
    #[command(subcommand)]
    Datasets(DatasetCommands),

    /// Configuration commands
    #[command(subcommand)]
    Config(ConfigCommands),
}

#[derive(Subcommand)]
enum AuthCommands {
    /// Login to Zere platform
    Login {
        /// Email address
        #[arg(long)]
        email: Option<String>,

        /// Password
        #[arg(long)]
        password: Option<String>,

        /// API key (alternative to email/password)
        #[arg(long)]
        api_key: Option<String>,
    },

    /// Logout from Zere platform
    Logout,

    /// Show current user information
    Whoami,
}

#[derive(Subcommand)]
enum AssetCommands {
    /// List all assets
    List,

    /// Upload an asset
    Upload {
        /// Path to the asset file
        file: PathBuf,

        /// Asset name (defaults to filename)
        #[arg(short, long)]
        name: Option<String>,

        /// Asset category
        #[arg(short, long)]
        category: Option<String>,

        /// Tags (can be specified multiple times)
        #[arg(short, long)]
        tags: Vec<String>,
    },

    /// Get asset information
    Info {
        /// Asset ID
        asset_id: String,
    },

    /// Delete an asset
    Delete {
        /// Asset ID
        asset_id: String,

        /// Force deletion without confirmation
        #[arg(short, long)]
        force: bool,
    },
}

#[derive(Subcommand)]
enum JobCommands {
    /// List all jobs
    List,

    /// Create a new job
    Create {
        /// Job name
        #[arg(short, long)]
        name: String,

        /// Number of scenes to generate
        #[arg(short = 's', long)]
        num_scenes: i32,

        /// Path to config YAML file
        #[arg(short, long)]
        config: Option<PathBuf>,
    },

    /// Quick Start - Generate with procedural objects (no assets needed)
    QuickStart {
        /// Number of scenes to generate
        #[arg(short = 's', long)]
        num_scenes: i32,

        /// Object count range (e.g., "20-30")
        #[arg(short, long, default_value = "25-35")]
        objects: String,

        /// Environment type (warehouse_shelf, floor, table)
        #[arg(short, long, default_value = "warehouse_shelf")]
        environment: String,
    },

    /// Get job status
    Status {
        /// Job ID
        job_id: String,
    },

    /// Watch job progress in real-time
    Watch {
        /// Job ID
        job_id: String,
    },

    /// Cancel a running job
    Cancel {
        /// Job ID
        job_id: String,

        /// Force cancellation without confirmation
        #[arg(short, long)]
        force: bool,
    },
}

#[derive(Subcommand)]
enum DatasetCommands {
    /// List all datasets
    List,

    /// Get dataset information
    Info {
        /// Dataset ID
        dataset_id: String,
    },

    /// Download a dataset
    Download {
        /// Job ID
        job_id: String,

        /// Output path (defaults to {job_id}.zip)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

#[derive(Subcommand)]
enum ConfigCommands {
    /// Set API endpoint
    SetEndpoint {
        /// API endpoint URL
        endpoint: String,
    },

    /// Show current configuration
    Show,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Some(Commands::Auth(auth_cmd)) => match auth_cmd {
            AuthCommands::Login { email, password, api_key } => {
                commands::auth::login(email, password, api_key).await
            }
            AuthCommands::Logout => commands::auth::logout().await,
            AuthCommands::Whoami => commands::auth::whoami().await,
        },
        Some(Commands::Login { email, password, api_key }) => {
            commands::auth::login(email, password, api_key).await
        }
        Some(Commands::Logout) => commands::auth::logout().await,
        Some(Commands::Whoami) => commands::auth::whoami().await,
        Some(Commands::Assets(asset_cmd)) => match asset_cmd {
            AssetCommands::List => commands::assets::list().await,
            AssetCommands::Upload { file, name, category, tags } => {
                commands::assets::upload(file, name, category, tags).await
            }
            AssetCommands::Info { asset_id } => commands::assets::info(asset_id).await,
            AssetCommands::Delete { asset_id, force } => {
                commands::assets::delete(asset_id, force).await
            }
        },
        Some(Commands::Jobs(job_cmd)) => match job_cmd {
            JobCommands::List => commands::jobs::list().await,
            JobCommands::Create { name, num_scenes, config } => {
                commands::jobs::create(name, num_scenes, config).await
            }
            JobCommands::QuickStart { num_scenes, objects, environment } => {
                commands::jobs::quick_start(num_scenes, Some(objects), Some(environment)).await
            }
            JobCommands::Status { job_id } => commands::jobs::status(job_id).await,
            JobCommands::Watch { job_id } => commands::jobs::watch(job_id).await,
            JobCommands::Cancel { job_id, force } => commands::jobs::cancel(job_id, force).await,
        },
        Some(Commands::Datasets(dataset_cmd)) => match dataset_cmd {
            DatasetCommands::List => commands::datasets::list().await,
            DatasetCommands::Info { dataset_id } => commands::datasets::info(dataset_id).await,
            DatasetCommands::Download { job_id, output } => {
                commands::datasets::download(job_id, output).await
            }
        },
        Some(Commands::Config(config_cmd)) => match config_cmd {
            ConfigCommands::SetEndpoint { endpoint } => {
                commands::config::set_endpoint(endpoint).await
            }
            ConfigCommands::Show => commands::config::show().await,
        },
        None => {
            if cli.interactive {
                // Launch TUI mode
                tui::run().await
            } else {
                // Show help
                println!("{}", style("Zere CLI - Synthetic data generation for robotics").bold());
                println!();
                println!("Run 'zere --help' for usage information");
                println!("Run 'zere --interactive' to launch TUI mode");
                Ok(())
            }
        }
    };

    if let Err(e) = result {
        eprintln!("{} {}", style("Error:").red().bold(), e);
        std::process::exit(1);
    }
}
