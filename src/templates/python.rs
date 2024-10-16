// TODO - Take image_name, app_name and CMD from user input or read the root file and package file
// TODO - Check for root file for python (is it app.py or something else)
// TODO - Check for flask and django project and check for respective run CMD

pub fn python_dkfl<'a>(image_name: &'a str, _app_name: &'a str, work_dir: &'a str) -> String {
    let python_dkfl_template = format!(
        r#"
# Stage 1: Build the Python application
FROM {image_name} AS builder

# Set the working directory
WORKDIR {work_dir}

# Copy and install dependencies
COPY requirements.txt ./
RUN pip install --no-cache-dir -r requirements.txt

# Copy application files
COPY . .

# Stage 2: Final runtime image
FROM python:3.11-alpine

# Set the working directory
WORKDIR {work_dir}

# Copy the application from the build stage
COPY --from=builder {work_dir} {work_dir}

# Command to run the Python application
CMD ["python", "app.py"]
    "#
    );

    python_dkfl_template.to_owned()
}
