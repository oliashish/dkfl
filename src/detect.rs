use std::collections::{HashMap, HashSet};
// use std::process::exit;

use log::{error, info, warn};

// Assuming these are your enums and structs
use crate::enums::{Filetype, Projects};

pub fn detect_project(
    dir: Result<impl Iterator<Item = Result<std::fs::DirEntry, std::io::Error>>, std::io::Error>,
) -> Result<Projects, std::io::Error> {
    let dir_entries = dir?;

    // Collect file names into a HashSet for efficient lookup
    let file_names: HashSet<String> = dir_entries
        .filter_map(|entry_result| match entry_result {
            Ok(entry) => entry.file_name().to_str().map(String::from),
            Err(err) => {
                error!("Error reading directory entry: {:?}", err);
                None
            }
        })
        .collect();

    // Map file names to Projects variants and messages
    let project_map: HashMap<&str, (Projects, &str)> = [
        (
            Filetype::BuildGradle.to_str(),
            (Projects::GradleJava, "Gradle Java Project Detected"),
        ),
        (
            Filetype::PomXml.to_str(),
            (Projects::MavenJava, "Maven Java Project Detected"),
        ),
        (
            Filetype::PackageJson.to_str(),
            (Projects::Nodejs, "NodeJS Project Detected"),
        ),
        (
            Filetype::Gemfile.to_str(),
            (Projects::Ruby, "Ruby on Rails Project Detected"),
        ),
        (
            Filetype::CargoToml.to_str(),
            (Projects::Rust, "Rust Project Detected"),
        ),
        (
            Filetype::RequirementsTxt.to_str(),
            (Projects::Python, "Python Project Detected"),
        ),
        (
            Filetype::ComposerJson.to_str(),
            (Projects::Php, "PHP Project Detected"),
        ),
        (
            Filetype::GoSum.to_str(),
            (Projects::Go, "Go Project Detected"),
        ),
    ]
    .iter()
    .cloned()
    .collect();

    // Find the project based on the presence of specific files
    let project = file_names
        .iter()
        .find_map(|file_name| {
            project_map
                .get(file_name.as_str())
                .map(|&(project, message)| {
                    info!("{}", message);
                    project
                })
        })
        .unwrap_or_else(|| {
            warn!("Unknown or Unsupported Project Detected");
            Projects::Unknown
        });

    Ok(project)
}
