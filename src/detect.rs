use crate::enums::{self, Projects};
use anyhow::Error;
use std::{fs, path::PathBuf, process::exit};

pub fn detect_project(path: &PathBuf) -> Result<enums::Projects, Error> {
    let dir = fs::read_dir(path);

    // let project =
    match dir {
        Ok(dir) => {
            let mut files = Vec::new();
            for entry in dir {
                let dir_entry = entry?;
                if let Some(file_name) = dir_entry.file_name().to_str() {
                    files.push(file_name.to_string());
                };
            }

            let project = match true {
                _ if files.contains(&enums::Filetype::BuildGradle.to_str().to_owned()) => {
                    println!("Gradle Java Projoct Detected");
                    Projects::GradleJava
                }
                _ if files.contains(&enums::Filetype::PomXml.to_str().to_owned()) => {
                    println!("Maven Java Projoct Detected");

                    Projects::MavenJava
                }
                _ if files.contains(&enums::Filetype::PackageJson.to_str().to_owned()) => {
                    println!("NodeJS Projoct Detected");

                    Projects::Nodejs
                }
                _ if files.contains(&enums::Filetype::Gemfile.to_str().to_owned()) => {
                    println!("Ruby on rails Projoct Detected");

                    Projects::Ruby
                }
                _ if files.contains(&enums::Filetype::CargoToml.to_str().to_owned()) => {
                    println!("Rust Projoct Detected");

                    Projects::Rust
                }
                _ if files.contains(&enums::Filetype::RequirementsTxt.to_str().to_owned()) => {
                    println!("Python Projoct Detected");

                    Projects::Python
                }
                _ if files.contains(&enums::Filetype::ComposerJson.to_str().to_owned()) => {
                    println!("Php Projoct Detected");

                    Projects::Php
                }
                _ if files.contains(&enums::Filetype::GoSum.to_str().to_owned()) => Projects::Go,
                _ => {
                    println!("Unknown or Unsupported Projoct Detected");
                    Projects::Unknown
                }
            };
            return Ok(project);
        }
        Err(err) => {
            println!("Error while reading directory: {:?}", err);
            exit(1)
        }
    };
}
