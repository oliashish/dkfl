use clap::{arg, Command};

pub fn push() -> Command {
    Command::new("push")
        .about("Push your created docker image to your registry")
        .arg(arg!(<REMOTE> "Your repository URL"))
        .arg_required_else_help(true)
}
