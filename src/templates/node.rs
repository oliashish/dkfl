// TODO - Take image_name, app_name and CMD from user input or read the root file and package file

use super::DockerConfig;

pub fn node_dkfl(config: DockerConfig) -> String {
    // Cannot use vector in format string
    let cmd_str = config
        .cmd
        .iter()
        .map(|part| format!(r#""{}""#, part))
        .collect::<Vec<String>>()
        .join(", ");

    let dockerfile = format!(
        r#"
# Stage 1: Build the Node project
FROM node:{app_version}

# Set the working directory inside the container
WORKDIR {work_dir}/{app_name}

# Now copy the actual source code to working directory in container
COPY . .

# Install the project dependencies
RUN npm install


# Specify the command to run the application
CMD ["{cmd}"]
    "#,
        app_version = config.app_version,
        app_name = config.app_name,
        work_dir = config.work_dir,
        cmd = cmd_str,
    );

    dockerfile
}
