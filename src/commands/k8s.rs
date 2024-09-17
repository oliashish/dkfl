use clap::Command;

pub fn k8s() -> Command {
    Command::new("k8s").about("Manifest k8s yaml file")
    // .arg(arg!(<PATH> "Path to dockerfile"))
    // .arg_required_else_help(true)
}
