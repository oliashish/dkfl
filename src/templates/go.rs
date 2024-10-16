// TODO - Take image_name, app_name and CMD from user input or read the root file and package file

pub fn go_dkfl<'a>(image_name: &'a str, app_name: &'a str, work_dir: &'a str) -> String {
    let go_dkfl_template = format!(
        r#"
# Stage 1: Build the Go binary
FROM {image_name} AS builder

# Set the Current Working Directory inside the container
WORKDIR {work_dir}

# Copy go.mod and go.sum files first (for dependency caching)
COPY go.mod go.sum ./

# Download all dependencies. Dependencies will be cached if the go.mod and go.sum files are not changed
RUN go mod download

# Copy the rest of the application code
COPY . .

# Build the Go app
RUN CGO_ENABLED=0 GOOS=linux GOARCH=amd64 go build -o {app_name} .

# Stage 2: Build a minimal final image
FROM alpine:latest

# Set the working directory in the final image
WORKDIR {work_dir}

# Copy the Go binary from the builder stage
COPY --from=builder {work_dir}/{app_name} .

# Command to run the executable
CMD ["{app_name}"]
    "#
    );

    go_dkfl_template.to_owned()
}
