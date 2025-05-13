use std::{fs, io::Write, path::PathBuf, process::exit};

use anyhow::{Context, Result};
use log::{debug, error};

use crate::{
    detect::detect_project,
    enums::Projects,
    templates::{
        generate_config, go::go_dkfl, gradle::gradle_dkfl, maven::maven_dkfl, node::node_dkfl,
        php::php_dkfl, python::python_dkfl, ruby::ruby_dkfl, rust::rust_dkfl,
    },
};

pub fn handle_create(
    path: &PathBuf,
    app_name: &str,
    work_dir: &str,
    app_version: &str,
    cmd: &str,
) -> Result<()> {
    let project = detect_project(path);

    let cmd = resolve_cmd(cmd, &project);
    let docker_config = generate_config(app_name, app_version, work_dir, cmd);

    let dockerfile = match project {
        Projects::Nodejs => node_dkfl(docker_config),
        Projects::Rust => rust_dkfl(docker_config),
        Projects::Go => go_dkfl(docker_config),
        Projects::Python => python_dkfl(docker_config),
        Projects::Ruby => ruby_dkfl(docker_config),
        Projects::Php => php_dkfl(docker_config),
        Projects::GradleJava => gradle_dkfl(docker_config),
        Projects::MavenJava => maven_dkfl(docker_config),
        Projects::Unknown => {
            error!("Error while checking project. Unknown or Unsupported Language or framework.");
            exit(1)
        }
    };

    match create_dockerfile(&dockerfile, path) {
        Ok(_) => (),
        Err(err) => {
            error!("Error while creating Dockerfile: {}", err)
        }
    }
    Ok(())
}

pub fn resolve_cmd(cmd: &str, project: &Projects) -> Vec<String> {
    let cmd_str = if cmd.trim().is_empty() {
        project.default_cmd()
    } else {
        cmd
    };

    cmd_str.split_whitespace().map(|c| c.to_string()).collect()
}

pub fn create_dockerfile(dockerfile: &str, path: &PathBuf) -> Result<()> {
    // Handle user typing path in two ways - with and without trailing slash(/) in the end
    if path.ends_with("/") {
        path.clone().pop();
    }

    let file_path = format!("{}/Dockerfile", path.to_str().unwrap_or_default());

    // create dockerfile at the path
    let mut file = fs::File::create(&file_path)
        .with_context(|| format!("Failed to create Dockerfile at {}", file_path))?;

    match file.write_all(dockerfile.as_bytes()) {
        Ok(_) => debug!(
            "Content of dockerfile written to: {} succcessfully",
            file_path
        ),
        Err(err) => {
            error!("Unable to write to Dockerfile: {}", err);
            exit(1)
        }
    };

    Ok(())
}
