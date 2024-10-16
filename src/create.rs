use std::{fs, io::Write, path::PathBuf};

pub fn create_dockerfile(dockerfile: &str, path: &mut PathBuf) -> Result<(), std::io::Error> {
    // Handle user typing path in two ways - with and without trailing slash(/) in the end
    if path.ends_with("/") {
        path.pop();
    }

    let dockerfile = dockerfile.to_owned();

    // TODO - handle unwrap()
    let file_path = format!("{}/Dockerfile", path.to_str().unwrap());

    // create dockerfile at the path
    let mut file = fs::File::create(file_path)?;

    // Write to the newly created file
    match file.write_all(dockerfile.as_bytes()) {
        Ok(_) => Ok(()),
        Err(err) => {
            println!(
                "unable to write template to dockerfile at path: {:?} with error: {}",
                path, err
            );
            Err(err)
        }
    }
}
