# dependencies-patch

A simple tool to patch cargo dependencies with command line commands.



# Todo List

- [x] Support for packages from `github`
- [ ] Support for packages from other URL
- [x] Support for packages from `crates-io` 
- [ ] Support for packages from other registries



# Installation

```sh
$ cargo install dependencies-patch
```

# Usage

The `Cargo.toml` for `example_project` looks like this:

```toml
[package]
edition = "2021"
name = "project1"
version = "0.1.0"

[workspace]

[dependencies]
log = "0.4"
```

Then run following commands to patch the `log` dependency to a git repository:

```sh
$ dependencies-patch --help
$ dependencies-patch -c /path/to/example_project -n log -t git --git-repo rust-lang/log
```

After running the command, the `Cargo.toml` will be updated to:

```toml
[package]
edition = "2021"
name = "project1"
version = "0.1.0"

[workspace]

[dependencies]
log = "0.4"

[patch.crates-io.log]
git = "https://github.com/rust-lang//log.git"
```



We can also patch the package to the local path by running the following commands:

```sh
$ dependencies-patch -c . -n log -t path --patch-path ../log
```

And the `Cargo.toml` will be updated to:

```toml
[package]
name = "helloworld"
version = "0.1.0"
edition = "2021"

[dependencies]
log = "0.4"

[patch.crates-io.log]
path = "../log"
```



More usages can be known by running `dependencies --help` command.

# Notes

The tool can only add patches to the `Cargo.toml` file. It don't support removing patches.
