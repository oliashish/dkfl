use std::process::exit;

use clap::Parser;
use commands::{App, Commands};
use create::handle_create;
use log::{error, info};

mod commands;
mod create;
mod detect;
mod enums;
mod templates;

fn main() {
    env_logger::init();
    info!("App started. To know more use <dkfl --help>");

    let app = App::parse();

    match &app.command {
        Commands::Create {
            path,
            app_name,
            work_dir,
            app_version,
            cmd,
        } => match handle_create(&path, &app_name, &work_dir, &app_version, &cmd) {
            Ok(_) => {
                info!("Dockerfile successfully created at: {:?}", path)
            }
            Err(err) => {
                error!("Error while creating Dockerfile: {}", err);
                exit(1)
            }
        },
    }
}
