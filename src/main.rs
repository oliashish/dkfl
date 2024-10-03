use clap::{Parser, Subcommand};
use std::path::PathBuf;

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
    let app = App::parse();

    match &app.command {
        Commands::Create { path } => {
            let project = detect::detect_project(path);
            println!("Project from main : {:?}", project)
        }
    }
}
