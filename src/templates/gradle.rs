use super::DockerConfig;

pub fn gradle_dkfl(config: DockerConfig) -> String {
    let cmd_str = config
        .cmd
        .iter()
        .map(|part| format!(r#""{}""#, part))
        .collect::<Vec<String>>()
        .join(", ");
    let dockerfile = format!(
        r#"
# Stage 1: Build the application using Gradle
FROM gradle:{app_version} AS builder

# Set the working directory
WORKDIR {work_dir}

# Copy the Gradle files and project source code
COPY build.gradle settings.gradle {work_dir}
COPY src {work_dir}/src

# Build the project
RUN gradle build --no-daemon

# Stage 2: Create a minimal runtime image
FROM  openjdk:{app_version}

# Set the working directory
WORKDIR {work_dir}/{app_name}

# Copy the JAR file from the build stage
COPY --from=builder {work_dir}/build/libs/*.jar {app_name}.jar

# Command to run the application
CMD [{cmd}]
    "#,
        app_version = config.app_version,
        app_name = config.app_name,
        work_dir = config.work_dir,
        cmd = cmd_str
    );

    dockerfile
}
