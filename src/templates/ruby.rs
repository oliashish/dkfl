use super::DockerConfig;

pub fn ruby_dkfl(config: DockerConfig) -> String {
    let cmd_str = config
        .cmd
        .iter()
        .map(|part| format!(r#""{}""#, part))
        .collect::<Vec<String>>()
        .join(", ");

    let ruby_dkfl_template = format!(
        r#"
# Stage 1: Build the Ruby application
FROM ruby:{app_version} AS builder

# Set the working directory
WORKDIR {work_dir}

# Copy Gemfile and install dependencies
COPY Gemfile Gemfile.lock ./
RUN bundle install --without development test

# Copy application files
COPY . .

# Stage 2: Final runtime image
FROM ruby:{app_version}

# Set the working directory
WORKDIR {work_dir}/{app_name}

# Copy the application from the build stage
COPY --from=builder {work_dir} .

# Command to run the Ruby application
CMD [{cmd}]
    "#,
        app_version = config.app_version,
        app_name = config.app_name,
        work_dir = config.work_dir,
        cmd = cmd_str
    );

    ruby_dkfl_template.to_owned()
}
