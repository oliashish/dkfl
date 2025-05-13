use std::path::Path;

use crate::enums::Projects;

pub trait ProjectDetector {
    fn detect_project(&self, dir: &Path) -> Option<Projects>;
}

pub struct NodejsDetector;

impl ProjectDetector for NodejsDetector {
    fn detect_project(&self, dir: &Path) -> Option<Projects> {
        if dir.join("package.json").exists() {
            Some(Projects::Nodejs)
        } else {
            None
        }
    }
}

pub struct GoDetector;

impl ProjectDetector for GoDetector {
    fn detect_project(&self, dir: &Path) -> Option<Projects> {
        if dir.join("go.mod").exists() {
            Some(Projects::Go)
        } else {
            None
        }
    }
}

pub struct RustDetector;

impl ProjectDetector for RustDetector {
    fn detect_project(&self, dir: &Path) -> Option<Projects> {
        if dir.join("Cargo.toml").exists() {
            Some(Projects::Rust)
        } else {
            None
        }
    }
}

pub struct PythonDetector;

impl ProjectDetector for PythonDetector {
    fn detect_project(&self, dir: &Path) -> Option<Projects> {
        if dir.join("requirements.txt").exists() {
            Some(Projects::Python)
        } else {
            None
        }
    }
}

pub struct RubyDetector;

impl ProjectDetector for RubyDetector {
    fn detect_project(&self, dir: &Path) -> Option<Projects> {
        if dir.join("Gemfile").exists() {
            Some(Projects::Ruby)
        } else {
            None
        }
    }
}

pub struct PhpDetector;

impl ProjectDetector for PhpDetector {
    fn detect_project(&self, dir: &Path) -> Option<Projects> {
        if dir.join("composer.json").exists() {
            Some(Projects::Php)
        } else {
            None
        }
    }
}

pub struct MavenJavaDetector;

impl ProjectDetector for MavenJavaDetector {
    fn detect_project(&self, dir: &Path) -> Option<Projects> {
        if dir.join("pom.xml").exists() {
            Some(Projects::MavenJava)
        } else {
            None
        }
    }
}

pub struct GradleJavaDetector;

impl ProjectDetector for GradleJavaDetector {
    fn detect_project(&self, dir: &Path) -> Option<Projects> {
        if dir.join("build.gradle").exists() {
            Some(Projects::GradleJava)
        } else {
            None
        }
    }
}

pub fn detect_project(path: &Path) -> Projects {
    let detectors: Vec<Box<dyn ProjectDetector>> = vec![
        Box::new(NodejsDetector),
        Box::new(GoDetector),
        Box::new(PythonDetector),
        Box::new(PhpDetector),
        Box::new(RustDetector),
        Box::new(MavenJavaDetector),
        Box::new(GradleJavaDetector),
    ];

    for detector in detectors {
        if let Some(project) = detector.detect_project(path) {
            return project;
        }
    }

    Projects::Unknown
}
