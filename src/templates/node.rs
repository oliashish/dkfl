// TODO - Take image_name, app_name and CMD from user input or read the root file and package file

pub fn node_dkfl<'a>(image_name: &'a str, _app_name: &'a str, work_dir: &'a str) -> String {
    let node_dkfl_template = format!(
        r#"
# Stage 1: Build the Node project
FROM {image_name} AS builder

# Set the working directory inside the container
WORKDIR {work_dir}

# Now copy the actual source code to working directory in container
COPY . .

# Install the project dependencies
RUN npm install


# Specify the command to run the application
CMD ["node", index.js]
    "#
    );

    node_dkfl_template.to_owned()
}
