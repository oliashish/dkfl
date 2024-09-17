use clap::{arg, Command};

pub fn detect() -> Command {
    Command::new("detect")
        .about("Detect your project")
        .arg(arg!(<PATH> "Path to project directory"))
        .arg_required_else_help(true)
}
