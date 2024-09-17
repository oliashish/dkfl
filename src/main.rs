// use std::ffi::OsString;
// use std::path::PathBuf;

use clap::Command;

pub mod commands;

fn main() {
    let build = commands::build::build();
    let generate = commands::generate::generate();
    let detect = commands::detect::detect();
    let k8s = commands::k8s::k8s();
    let push = commands::push::push();
    let app = Command::new("dkfl")
        .about("A CLI tool to automation your containerization and deployment")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(build)
        .subcommand(generate)
        .subcommand(detect)
        .subcommand(k8s)
        .subcommand(push);
    println!("{:?}", app.get_matches())
}
