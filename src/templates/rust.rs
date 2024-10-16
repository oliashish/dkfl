// TODO - Take image_name, app_name and CMD from user input or read the root file and package file

pub fn rust_dkfl<'a>(image_name: &'a str, app_name: &'a str, work_dir: &'a str) -> String {
    let rust_dkfl_template = format!(
        r#"
# Stage 1: Build the Rust project
FROM {image_name} AS builder

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
CMD ["./{app_name}"]
    "#
    );

    rust_dkfl_template.to_owned()
}
