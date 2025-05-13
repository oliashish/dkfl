pub mod go;
pub mod gradle;
pub mod maven;
pub mod node;
pub mod php;
pub mod python;
pub mod ruby;
pub mod rust;

pub struct DockerConfig {
    pub app_name: String,
    pub work_dir: String,
    pub app_version: String,
    pub cmd: Vec<String>,
}

pub fn generate_config(
    app_name: &str,
    app_version: &str,
    work_dir: &str,
    cmd: Vec<String>,
) -> DockerConfig {
    return DockerConfig {
        app_name: app_name.to_string(),
        work_dir: work_dir.to_string(),
        app_version: app_version.to_string(),
        cmd,
    };
}
