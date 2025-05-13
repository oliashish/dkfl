use super::DockerConfig;

pub fn php_dkfl(config: DockerConfig) -> String {
    let cmd_str = config
        .cmd
        .iter()
        .map(|part| format!(r#""{}""#, part))
        .collect::<Vec<String>>()
        .join(", ");
    let php_dkfl_template = format!(
        r#"
# Stage 1: Build the PHP application
FROM php:{app_version} AS builder

# Set the working directory
WORKDIR {work_dir}/{app_name}

# Copy composer files and install depende{app_version}ncies
COPY composer.json composer.lock ./
RUN curl -sS https://getcomposer.org/installer | php -- --install-dir=/usr/local/bin --filename=composer
RUN composer install --no-dev --optimize-autoloader

# Copy application files
COPY . .

# Stage 2: Final runtime image
FROM php:{app_version}

# Set the working directory
WORKDIR {work_dir}/{app_name}

# Copy the application from the build stage
COPY --from=builder {work_dir}/{app_name} {work_dir}/{app_name}

# Command to run the PHP application
CMD [{cmd}]
    "#,
        app_version = config.app_version,
        app_name = config.app_name,
        work_dir = config.work_dir,
        cmd = cmd_str
    );

    php_dkfl_template.to_owned()
}
