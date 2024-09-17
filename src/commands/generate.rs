use clap::Command;

pub fn generate() -> Command {
    Command::new("generate").about("Generate Dockerfile for your project")
}
