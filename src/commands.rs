use std::path::PathBuf;

use clap::{command, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "dkfl")]
#[command(about = "A CLI tool to automation your containerization and deployment")]
pub struct App {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a Dockerfile with automatically detecting the project based on give project path
    Create {
        /// Path of the project directory (defaults to .)
        #[arg(short = 'p', long, default_value = ".")]
        path: PathBuf,

        /// Name of the application
        #[arg(short = 'a', long, default_value = "myapp")]
        app_name: String,

        /// Working directory inside container
        #[arg(short = 'd', long, default_value = "myproject")]
        work_dir: String,

        /// Application version tag
        #[arg(short = 'v', long, default_value = "latest")]
        app_version: String,

        /// Custom command to run inside
        /// For languages not producing binary it has to array of command to run e.g ["node", "index.js"] or ["python3", "manage.py", "runserver"]
        #[arg(short = 'c', long, default_value = "")]
        cmd: String,
    },
}
