#[derive(Debug, Clone, Copy)]

pub enum Projects {
    Nodejs,
    Rust,
    Go,
    Python,
    Ruby,
    Php,
    GradleJava,
    MavenJava,
    Unknown,
}

impl Projects {
    pub fn default_cmd(&self) -> &str {
        match self {
            Projects::Nodejs => "npm start",
            Projects::Python => "python main.py",
            Projects::Go => "go run main.go",
            Projects::Rust => "cargo run",
            Projects::Ruby => "rails server",
            Projects::Php => "php index.php",
            Projects::GradleJava => "./gradlew bootRun",
            Projects::MavenJava => "mvn spring-boot:run",
            Projects::Unknown => "",
        }
    }
}

// TODO: Not removing, have to validate if not needed in future completely

// #[derive(Debug)]
// pub enum Filetype {
//     RequirementsTxt,
//     PackageJson,
//     Gemfile,
//     CargoToml,
//     ComposerJson,
//     PomXml,
//     BuildGradle,
//     GoSum,
// }

// impl Filetype {
//     pub fn to_str(&self) -> &'static str {
//         match self {
//             Filetype::BuildGradle => "build.gradle",
//             Filetype::PomXml => "pom.xml",
//             Filetype::PackageJson => "package.json",
//             Filetype::Gemfile => "Gemfile",
//             Filetype::CargoToml => "Cargo.toml",
//             Filetype::RequirementsTxt => "requirements.txt",
//             Filetype::ComposerJson => "composer.json",
//             Filetype::GoSum => "go.sum",
//         }
//     }
// }
