// TODO - Take image_name, app_name and CMD from user input or read the root file and package file
// TODO - Check for idiomatic root file or read dynamically from project (index.php)

pub fn php_dkfl<'a>(image_name: &'a str, _app_name: &'a str, work_dir: &'a str) -> String {
    let php_dkfl_template = format!(
        r#"
# Stage 1: Build the PHP application
FROM {image_name} AS builder

# Set the working directory
WORKDIR {work_dir}

# Copy composer files and install dependencies
COPY composer.json composer.lock ./
RUN curl -sS https://getcomposer.org/installer | php -- --install-dir=/usr/local/bin --filename=composer
RUN composer install --no-dev --optimize-autoloader

# Copy application files
COPY . .

# Stage 2: Final runtime image
FROM php:8.2-alpine

# Set the working directory
WORKDIR {work_dir}

# Copy the application from the build stage
COPY --from=builder {work_dir} {work_dir}

# Command to run the PHP application
CMD ["php", "index.php"]
    "#
    );

    php_dkfl_template.to_owned()
}
