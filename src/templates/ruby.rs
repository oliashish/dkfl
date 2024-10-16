// TODO - Take image_name, app_name and CMD from user input or read the root file and package file
// TODO - Check for idiomatic root file or read dynamically from project (app.rb)

pub fn ruby_dkfl<'a>(image_name: &'a str, _app_name: &'a str, work_dir: &'a str) -> String {
    let ruby_dkfl_template = format!(
        r#"
# Stage 1: Build the Ruby application
FROM {image_name} AS builder

# Set the working directory
WORKDIR {work_dir}

# Copy Gemfile and install dependencies
COPY Gemfile Gemfile.lock ./
RUN bundle install --without development test

# Copy application files
COPY . .

# Stage 2: Final runtime image
FROM ruby:3.2-alpine

# Set the working directory
WORKDIR {work_dir}

# Copy the application from the build stage
COPY --from=builder {work_dir} {work_dir}

# Command to run the Ruby application
CMD ["ruby", "app.rb"]
    "#
    );

    ruby_dkfl_template.to_owned()
}
