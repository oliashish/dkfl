use crate::enums;
use anyhow::Error;
use std::{fs, path::PathBuf, process::exit};

pub fn detect_project(path: &PathBuf) -> Result<enums::Projects, Error> {
    let dir = fs::read_dir(path);

    // let project =
    match dir {
        Ok(dir) => {
            for entry in dir {
                let dir_entry = entry?;
                println!("Directory entries: {:?}", dir_entry.path())
            }
        }
        Err(err) => {
            println!("Error while reading directory: {:?}", err);
            exit(1)
        }
    };

    return Ok(enums::Projects::Go);
}
