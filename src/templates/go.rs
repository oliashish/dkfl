use super::DockerConfig;

pub fn go_dkfl(config: DockerConfig) -> String {
    let cmd_str = config
        .cmd
        .iter()
        .map(|part| format!(r#""{}""#, part))
        .collect::<Vec<String>>()
        .join(", ");

    let dockerfile = format!(
        r#"# Stage 1: Build the Go binary
FROM golang:{app_version} AS builder

# Set the Current Working Directory inside the container
WORKDIR {work_dir}

# Copy go.mod and go.sum files first (for dependency caching)
COPY go.mod go.sum ./

# Download all dependencies. Dependencies will be cached if the go.mod and
COPY . .

# Build the Go app
RUN CGO_ENABLED=0 GOOS=linux GOARCH=amd64 go build -o {app_name} .

# Stage 2: Build a minimal final image(this is docker multi-stage build for    # smalled image size
FROM alpine:latest

# Set the working directory in the final image
WORKDIR {work_dir}/{app_name}

# Copy the Go binary from the builder stage
COPY --from=builder {work_dir} .

# Command to run the executable
CMD [{cmd}]
    "#,
        app_version = config.app_version,
        app_name = config.app_name,
        work_dir = config.work_dir,
        cmd = cmd_str
    );

    dockerfile
}
