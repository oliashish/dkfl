use clap::{Parser, Subcommand};
use log::{error, info};
use std::path::PathBuf;

mod create;
mod detect;
mod enums;
mod generate_dkfl;
mod templates;

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

            // handle result and call the generate_template function
            match project {
                Ok(project) => {
                    let dockerfile = generate_dkfl::generate_dockerfile(project);
                    // Call write to directory function
                    let _res = create::create_dockerfile(&dockerfile, &mut path.clone());
                }
                Err(err) => {
                    info!("Unable to detect language/framework error: {}", err);
                    error!("Unable to detect language/framework error: {}", err);
                }
            }

            // read the root project file and take information like version, app_name, cmd to run the production server etc
            // TODO
        }
    }
}
