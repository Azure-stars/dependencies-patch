use std::fs::OpenOptions;
use std::io::Write;
use toml::Table;

use crate::patch::gen_patch_table;

/// The patch git-target information
pub(crate) enum GitInfo {
    /// No specific information
    None,
    /// The commit hash
    Commit(String),
    /// The tag name
    Tag(String),
    /// The branch name
    Branch(String),
}

/// The patch information
pub struct GitPatch {
    /// The git repository name
    git: String,
    /// The real name of the package which may be renamed in the Cargo.toml
    package: Option<String>,
    /// The version of the patch
    version: Option<String>,
    /// The patch target information
    info: GitInfo,
}

impl GitPatch {
    /// Create a new git patch
    pub(crate) fn new(
        git: String,
        package: Option<String>,
        version: Option<String>,
        info: GitInfo,
    ) -> Self {
        Self {
            git,
            package,
            version,
            info,
        }
    }
}

/// To check if the git patch format is correct
///
/// - The git repo name should be in the format of `owner/repo`
///
/// # Return
///
/// - Ok((owner, repo)): The owner and repo name of the given patch
/// - Err(mes): The error message
fn check_git_patch_format<'a>(patch: &'a GitPatch) -> Result<(&'a str, &'a str), String> {
    let names = patch.git.split('/').collect::<Vec<&str>>();
    if names.len() != 2 {
        return Err(format!("{} is not a valid git repo name!", patch.git));
    }
    Ok((names[0], names[1]))
}

/// Patch the specific package to the git repository
///
/// # Arguments
///
/// - `cargo_path`: The path of the cargo project, where the Cargo.toml file is in
///
/// - `package_name`: The name of the package to be patched
///
/// - `patch`: The patch information
pub(crate) fn do_git_patch(cargo_path: &String, package_name: &String, patch: GitPatch) {
    // If the package has been renamed, the `package` field in the patch should be used because it is the real package name.
    let real_package_name = match &patch.package {
        Some(name) => name,
        None => package_name,
    };

    let names = match check_git_patch_format(&patch) {
        Ok(names) => names,
        Err(mes) => {
            error_log!("{}", mes);
            return;
        }
    };

    // The table which contains the patch information, will be written to the Cargo.toml
    let (mut toml_table, package_index) =
        if let Some(res) = gen_patch_table(cargo_path, package_name, real_package_name) {
            res
        } else {
            return;
        };
    // Add the extra '/' before the repo name to avoid the error
    // `points to the same source, but patches must point to different sources`
    let patch_git = format!("https://github.com/{}//{}.git", names.0, names.1);
    let patch_toml_table = toml_table.get_mut("patch").unwrap().as_table_mut().unwrap();
    let git_table = patch_toml_table
        .get_mut(&package_index)
        .unwrap()
        .as_table_mut()
        .unwrap();

    // The table which contains the patch information
    let mut patch_table = Table::new();
    patch_table.insert("git".to_string(), toml::Value::String(patch_git.clone()));
    if let Some(target_package) = patch.package {
        patch_table.insert("package".to_string(), toml::Value::String(target_package));
    }
    if let Some(version) = patch.version {
        patch_table.insert("version".to_string(), toml::Value::String(version));
    }
    match patch.info {
        GitInfo::Commit(commit) => {
            patch_table.insert("rev".to_string(), toml::Value::String(commit));
        }
        GitInfo::Branch(branch) => {
            patch_table.insert("branch".to_string(), toml::Value::String(branch));
        }
        GitInfo::Tag(tag) => {
            patch_table.insert("tag".to_string(), toml::Value::String(tag));
        }
        GitInfo::None => {}
    }

    git_table.insert(package_name.clone(), toml::Value::Table(patch_table));
    // Write the patch table to the Cargo.toml in appending mode
    let mut file = OpenOptions::new()
        .append(true)
        .open(format!("{}/Cargo.toml", cargo_path))
        .unwrap();

    if let Err(mes) = file.write_all(toml::to_string(&toml_table).unwrap().as_bytes()) {
        error_log!("{}", mes);
    };
}
