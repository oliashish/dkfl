use clap::{Parser, Subcommand};
use log::{error, info};
use std::path::{Path, PathBuf};

mod detect;
mod enums;

#[derive(Parser)]
#[command(name = "dkfl")]
#[command(about = "A CLI tool to automation your containerization and deployment")]
struct App {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a Dockerfile with automatically detecting the project based on give project path
    Create {
        /// Path of the project directory (defaults to .)
        #[arg(short = 'p', long, default_value = ".")]
        path: PathBuf,
    },
}

fn main() {
    env_logger::init();

    let app = App::parse();

    match &app.command {
        Commands::Create { path } => {
            let dir = std::fs::read_dir(path);
            let project = detect::detect_project(dir);

            // read the root project file and take information like version, app_name, cmd to run the production server etc

            info!("Project from main : {:?}", project);
        }
    }
}
