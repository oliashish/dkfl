#[derive(Debug)]

pub enum Projects {
    Nodejs,
    Rust,
    Go,
    Python,
    Ruby,
    Php,
    Kotlin,
    Java,
}

#[derive(Debug)]
pub enum Filetype {
    RequirementsTxt,
    PackageJson,
    Gemfile,
    CargoToml,
    ComposerJson,
    PomXml,
    BuildGradle,
}

impl Filetype {
    fn to_str(&self) -> &'static str {
        match self {
            Filetype::BuildGradle => "build.gradle",
            Filetype::PomXml => "pom.xml",
            Filetype::PackageJson => "package.json",
            Filetype::Gemfile => "Gemfile",
            Filetype::CargoToml => "Cargo.toml",
            Filetype::RequirementsTxt => "requirements.txt",
            Filetype::ComposerJson => "composer.json",
        }
    }
}
