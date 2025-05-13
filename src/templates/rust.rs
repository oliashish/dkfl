// TODO - Take image_name, app_name and CMD from user input or read the root file and package file

use super::DockerConfig;

pub fn rust_dkfl(config: DockerConfig) -> String {
    // Cannot use vector in format string
    let cmd_str = config
        .cmd
        .iter()
        .map(|part| format!(r#""{}""#, part))
        .collect::<Vec<String>>()
        .join(", ");

    let dockerfile = format!(
        r#"
# Stage 1: Build the Rust project
FROM rust:{app_version} AS builder

# Set the working directory inside the container
WORKDIR {work_dir}

# Copy Cargo.toml and Cargo.lock files to the container to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Now copy the actual source code
COPY . .

# Build the project with optimizations
RUN cargo build --release --bin {app_name}



# Stage 2: Create a small image with only the binary
FROM debian:buster-slim

# Set the working directory for the runtime environment
WORKDIR {work_dir}

# Copy the compiled binary from the builder stage
COPY --from=builder {work_dir}/target/release/{app_name} .

# Specify the command to run the application
CMD [{cmd}]
    "#,
        app_version = config.app_version,
        app_name = config.app_name,
        work_dir = config.work_dir,
        cmd = cmd_str,
    );

    dockerfile
}
