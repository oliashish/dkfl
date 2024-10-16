use crate::enums::Projects;
use crate::templates;

pub fn generate_dockerfile(project: Projects) -> String {
    match project {
        // TOdo - Create templates for all of these
        Projects::Nodejs => todo!(),
        // TODO - Test params, remove it and take user provided parameters
        Projects::Rust => templates::rust::rust_dkfl("rust:latest", "dkfl", "/home"),
        Projects::Go => todo!(),
        Projects::Python => todo!(),
        Projects::Ruby => todo!(),
        Projects::Php => todo!(),
        Projects::GradleJava => todo!(),
        Projects::MavenJava => todo!(),
        Projects::Unknown => todo!(),
    }
}
