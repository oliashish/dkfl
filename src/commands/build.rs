use clap::{arg, Command};

pub fn build() -> Command {
    Command::new("build")
        .about("Build your created Dockerfile")
        .arg(arg!(<PATH> "Path to dockerfile"))
        .arg_required_else_help(true)
}
