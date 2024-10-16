// TODO - Take image_name, app_name and CMD from user input or read the root file and package file

pub fn gradle_dkfl<'a>(image_name: &'a str, app_name: &'a str, work_dir: &'a str) -> String {
    let gradle_dkfl_template = format!(
        r#"
# Stage 1: Build the application using Gradle
FROM {image_name} AS builder

# Set the working directory
WORKDIR {work_dir}

# Copy the Gradle files and project source code
COPY build.gradle settings.gradle {work_dir}
COPY src {work_dir}/src

# Build the project
RUN gradle build --no-daemon

# Stage 2: Create a minimal runtime image
FROM eclipse-temurin:17-jre-alpine

# Set the working directory
WORKDIR {work_dir}

# Copy the JAR file from the build stage
COPY --from=builder {work_dir}/build/libs/*.jar {app_name}.jar

# Command to run the application
CMD ["java", "-jar", "{app_name}.jar"]
    "#
    );

    gradle_dkfl_template.to_owned()
}
