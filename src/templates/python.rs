use super::DockerConfig;

pub fn python_dkfl(config: DockerConfig) -> String {
    let cmd_str = config
        .cmd
        .iter()
        .map(|part| format!(r#""{}""#, part))
        .collect::<Vec<String>>()
        .join(", ");

    let python_dkfl_template = format!(
        r#"
# Stage 1: Build the Python application
FROM python:{app_version} AS builder

# Set the working directory
WORKDIR {work_dir}

# Copy and install dependencies
COPY requirements.txt ./
RUN pip install --no-cache-dir -r requirements.txt

# Copy application files
COPY . .

# Stage 2: Final runtime image
FROM python:{app_version}

# Set the working directory
WORKDIR {work_dir}/{app_name}

# Copy the application from the build stage
COPY --from=builder {work_dir} .

# Command to run the Python application
CMD [{cmd}]
    "#,
        app_version = config.app_version,
        app_name = config.app_name,
        work_dir = config.work_dir,
        cmd = cmd_str
    );

    python_dkfl_template.to_owned()
}
