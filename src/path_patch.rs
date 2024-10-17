//! Do patch as local path

use std::{fs::OpenOptions, io::Write};

use toml::Table;

use crate::patch::gen_patch_table;

/// The information for path patch
pub struct PathPatch {
    /// The real name of the package which may be renamed in the Cargo.toml
    package: Option<String>,
    /// The target path of the patch
    path: String,
}

impl PathPatch {
    /// Create a new index patch
    pub fn new(package: Option<String>, path: String) -> Self {
        Self { package, path }
    }
}

/// Patch the specific package to the git repository
///
/// # Arguments
///
/// - `cargo_path`: The path of the cargo project, where the Cargo.toml file is in
///
/// - `package_name`: The name of the package to be patched
///
/// - `patch`: The PathPatch struct which contains the patch information
pub(crate) fn do_path_patch(cargo_path: &String, package_name: &String, patch: PathPatch) {
    // If the package has been renamed, the `package` field in the patch should be used because it is the real package name.
    let real_package_name = match &patch.package {
        Some(name) => name,
        None => package_name,
    };

    // The table which contains the patch information, will be written to the Cargo.toml
    let (mut toml_table, package_index) =
        if let Some(res) = gen_patch_table(cargo_path, package_name, real_package_name) {
            res
        } else {
            return;
        };

    let patch_toml_table = toml_table.get_mut("patch").unwrap().as_table_mut().unwrap();
    let index_table = patch_toml_table
        .get_mut(&package_index)
        .unwrap()
        .as_table_mut()
        .unwrap();

    // The table which contains the patch information
    let mut patch_table = Table::new();

    patch_table.insert("path".to_string(), toml::Value::String(patch.path.clone()));

    index_table.insert(package_name.clone(), toml::Value::Table(patch_table));
    // Write the patch table to the Cargo.toml in appending mode
    let mut file = OpenOptions::new()
        .append(true)
        .open(format!("{}/Cargo.toml", cargo_path))
        .unwrap();
    file.write_all("\n".as_bytes()).unwrap();
    if let Err(mes) = file.write_all(toml::to_string(&toml_table).unwrap().as_bytes()) {
        error_log!("{}", mes);
    };
}
