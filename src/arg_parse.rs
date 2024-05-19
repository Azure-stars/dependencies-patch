//! To parse the arguments of the command line
use clap::Parser;

/// A simple tool to patch cargo dependencies by command line
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
pub struct Args {
    /// The path of the cargo project, where the Cargo.toml file is in
    #[arg(short, long)]
    pub cargo_path: String,

    /// The name of the package to be patched, which may be renamed
    #[arg(short = 'n', long = "name")]
    pub package_name: String,

    /// The type of the patch, which can be `git`, `registry` or `path`
    ///
    /// - `git`: Patch the package to a git repository
    ///
    /// - `registry`: Patch the package to a registry
    ///
    /// - `path`: Patch the package to a local path
    ///
    /// `Notes`:
    /// - The `registry` only supports `crates-io`
    ///
    /// - The `git` points to `github.com` defaultly
    #[arg(short = 't', long = "type")]
    pub patch_type: String,

    /// The real package name to be patched
    #[arg(short = 'r', long = "real-package-name")]
    pub real_package_name: Option<String>,

    /// The version requirement for the target patch
    #[arg(short = 'v', long = "version")]
    pub package_version: Option<String>,

    /// The name of git repository to be patched for git patch
    ///
    /// The format should be like `owner/repo`
    #[arg(long)]
    pub git_repo: Option<String>,

    /// The commit hash to be patched for git patch
    #[arg(long)]
    pub commit: Option<String>,

    /// The tag name to be patched for git patch
    #[arg(long)]
    pub tag: Option<String>,

    /// The branch name to be patched for git patch
    #[arg(long)]
    pub branch: Option<String>,

    /// The local path to be patched for path patch
    #[arg(long)]
    pub patch_path: Option<String>,
}

/// To parse the arguments of the command line
///
/// If arguments are not valid, for example, the type is set as `git` but the git repo is not provided,
/// then return None.
///
/// # Return
//
/// - Some(args): The parsed arguments
///
/// - None: The arguments are not valid
pub(crate) fn parse_args() -> Option<Args> {
    let args = Args::parse();
    match args.patch_type.as_str() {
        "git" => {
            if args.git_repo.is_none() {
                error_log!("The git repo is required for git patch!");
                return None;
            }

            // commit, branch and tag can't be used with each other
            let judge_array = [
                args.commit.as_ref(),
                args.branch.as_ref(),
                args.tag.as_ref(),
            ];
            if judge_array.iter().filter(|x| x.is_some()).count() > 1 {
                error_log!("The commit, branch and tag can't be used with each other!");
                return None;
            }
        }
        "path" => {
            if args.patch_path.is_none() {
                error_log!("The path is required for path patch!");
                return None;
            }
        }
        "registry" => {
            if args.package_version.is_none() {
                error_log!("The version is required for registry patch!");
                return None;
            }
        }
        _ => {
            error_log!("Unsupported patch type: {}", args.patch_type);
            return None;
        }
    }
    Some(args)
}
