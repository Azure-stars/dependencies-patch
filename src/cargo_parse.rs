//! To parse the cargo dependencies of the target project

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum Dependency {
    Git(String),
    Path,
    Registry(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct CargoPackage {
    pub name: String,
    pub source: Option<String>,
}

impl CargoPackage {
    pub fn parse_dependency(&self) -> Dependency {
        match &self.source {
            Some(source) => {
                if source.starts_with("git+") {
                    // Get the git repo url between '+' and '#'
                    let git_url = source
                        .split('#')
                        .collect::<Vec<&str>>()
                        .get(0)
                        .unwrap()
                        .to_string()
                        .split_off(4);
                    if let Some(pos) = git_url.find('?') {
                        Dependency::Git(git_url[..pos].to_string());
                    }
                    Dependency::Git(git_url)
                } else if source.starts_with("registry+") {
                    // TODO: support more registries
                    Dependency::Registry("crates-io".to_string())
                } else {
                    panic!("Unsupported source: {}", source)
                }
            }
            None => Dependency::Path,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct CargoLock {
    package: Vec<CargoPackage>,
}

pub(crate) fn pick_package(
    cargo_path: &String,
    package_name: &String,
) -> Result<CargoPackage, String> {
    // Check if the Cargo.toml file exists
    let cargo_toml_path = format!("{}/Cargo.toml", cargo_path);
    if !std::path::Path::new(&cargo_toml_path).exists() {
        return Err(format!(
            "The Cargo.toml file is not found in {}",
            cargo_path
        ));
    }
    let cargo_lock_path = format!("{}/Cargo.lock", cargo_path);
    // Check if the Cargo.lock file exists
    if !std::path::Path::new(&cargo_lock_path).exists() {
        warn_log!("It will create a new Cargo.lock file");
        // Execute `cargo generate-lockfile` to generate the Cargo.lock file
        let output = std::process::Command::new("cargo")
            .arg("generate-lockfile")
            .current_dir(cargo_path)
            .spawn()
            .expect("Failed to execute command cargo generate-lockfile")
            .wait();
        if output.is_err() || !output.unwrap().success() {
            return Err(format!(
                "Failed to read the Cargo.lock file in {}",
                cargo_path
            ));
        }
    }
    // Check if the package exists in the Cargo.lock file
    let cargo_lock: CargoLock = toml::from_str(&std::fs::read_to_string(&cargo_lock_path).unwrap())
        .expect("Failed to parse Cargo.lock file");

    cargo_lock
        .package
        .iter()
        .find(|package| package.name == *package_name)
        .map(|package| package.clone())
        .ok_or(format!(
            "The package {} is not found in the Cargo.lock file",
            package_name
        ))
}
