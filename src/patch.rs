use std::fs;

use toml::Table;

use crate::{
    arg_parse::Args,
    cargo_parse::{pick_package, Dependency},
    git_patch::{self, GitInfo, GitPatch},
    index_patch::{self, IndexPatch},
};

/// Check whether the patch exists for the specific package
fn check_patch_exist(
    cargo_path: &String,
    package_name: &String,
    package_dependency: &Dependency,
) -> bool {
    let cargo_toml_path = format!("{}/Cargo.toml", cargo_path);
    let cargo_toml = fs::read_to_string(cargo_toml_path).unwrap();
    let cargo_toml: Table = toml::from_str(&cargo_toml).unwrap();
    if !cargo_toml.contains_key("patch") {
        return false;
    }
    let patch_table = cargo_toml.get("patch").unwrap().as_table().unwrap();
    if patch_table.contains_key(package_name) {
        return true;
    }

    match package_dependency {
        Dependency::Git(git) => patch_table.contains_key(git),
        Dependency::Registry(_) => {
            // TODO: only support crates-io now
            let registry_table = patch_table.get("crates-io").unwrap().as_table().unwrap();
            registry_table.contains_key(package_name)
        }
        _ => false,
    }
}

/// To generate the patch table by Cargo.lock
///
/// # Arguments
///
/// - `cargo_path`: The path of the cargo project, where the Cargo.toml and Cargo.lock are in
///
/// - `package_name`: The name of the package to be patched, which may be renamed
///
/// - `real_package_name`: The real package name to be patched
///
/// # Return
///
/// - Some((patch_table, package_index)):
///    - patch_table: The patch table to be written into the Cargo.toml
///    - package_index: The URL or registry name of the package set in the Cargo.lock
pub(crate) fn gen_patch_table(
    cargo_path: &String,
    package_name: &String,
    real_package_name: &String,
) -> Option<(Table, String)> {
    // If the package has been renamed, the `package` field in the patch should be used
    // because it is the real package name.
    let package = match pick_package(cargo_path, real_package_name) {
        Ok(package) => package,
        Err(mes) => {
            error_log!("{}", mes);
            return None;
        }
    };
    let package_dependency = package.parse_dependency();

    // But when do patch, we should use the original package name whether it has been renamed or not.
    if check_patch_exist(cargo_path, package_name, &package_dependency) {
        error_log!(
            "The patch for package {} already exists! Do nothing!",
            package_name
        );
        return None;
    }

    let mut cargo_toml: Table = Table::new();

    cargo_toml.insert(String::from("patch"), toml::Value::Table(Table::new()));

    let patch_table = cargo_toml.get_mut("patch").unwrap().as_table_mut().unwrap();

    match package_dependency {
        Dependency::Git(git) => {
            patch_table.insert(git.to_string(), toml::Value::Table(Table::new()));
            Some((cargo_toml, git.to_string()))
        }
        Dependency::Registry(_) => {
            // TODO: only support crates-io now
            patch_table.insert("crates-io".to_string(), toml::Value::Table(Table::new()));
            Some((cargo_toml, "crates-io".to_string()))
        }
        _ => {
            error_log!("The package specified is a path dependency, which can't be patched!");
            return None;
        }
    }
}

pub(crate) fn patch(args: Args) {
    match args.patch_type.as_str() {
        "git" => {
            let mut git_info = GitInfo::None;
            if let Some(commit) = &args.commit {
                git_info = GitInfo::Commit(commit.to_string());
            } else if let Some(tag) = &args.tag {
                git_info = GitInfo::Tag(tag.to_string());
            } else if let Some(branch) = &args.branch {
                git_info = GitInfo::Branch(branch.to_string());
            }
            let git_patch = GitPatch::new(
                args.git_repo.unwrap(),
                args.real_package_name,
                args.package_version,
                git_info,
            );
            git_patch::do_git_patch(&args.cargo_path, &args.package_name, git_patch);
        }
        // "path" => {
        //     // patch(&args.cargo_path, &args.package_name, &args.patch_path.unwrap());
        // }
        "registry" => {
            let index_patch =
                IndexPatch::new(args.real_package_name, args.package_version.unwrap());
            index_patch::do_index_patch(&args.cargo_path, &args.package_name, &index_patch);
        }
        _ => {
            error_log!("Unsupported patch type: {}", args.patch_type);
            return;
        }
    }
}
