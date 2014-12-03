//! xdg-rs is a utility library to make conforming to the [XDG specification](http://standards.freedesktop.org/basedir-spec/basedir-spec-latest.html) easier
//!
//! Some code borrowed from [rust-xdg](https://github.com/o11c/rust-xdg). ```rust-xdg``` is
//! currently a more complete implementation of the specification. The APIs provided by
//! ```rust-xdg``` and ```xdg-rs``` are different.

#![feature(if_let)]

use std::io;
use std::io::fs::PathExtensions;
use std::os;

/// Run some sanity checks for the XDG spec
///
/// Returns ```Ok(true)``` if $XDG_RUNTIME_DIR is founc and valid, ```Ok(false)``` if not found
/// and ```Err(msg)``` if and error occured
pub fn xdg_init() -> Result<bool, String> {
    match getenv_path("XDG_RUNTIME_DIR") {
        Some(path) => {
            match path.stat() {
                Ok(stat) => {
                    if stat.perm.intersects(io::GROUP_RWX | io::OTHER_RWX) {
                        Err("Incorrect permissions for $XDG_RUNTIME_DIR".to_string())
                    } else {
                        Ok(true)
                    }
                },

                Err(_) => Ok(false)
            }
        },

        None => Ok(false),
    }
}

/// Get the data home directory
///
/// If $XDG_DATA_HOME is not set, it returns $HOME/.local/share
pub fn get_data_home() -> Path {
    getenv_path("XDG_DATA_HOME").unwrap_or(
        os::homedir().unwrap().join("/.local/share")
    )
}

/// Get the config home directory
///
/// If $XDG_CONFIG_HOME is not set, it returns $HOME/.config
pub fn get_config_home() -> Path {
    getenv_path("XDG_CONFIG_HOME").unwrap_or(
        os::homedir().unwrap().join("/.config")
    )
}

/// Get the cache home directory
///
/// If $XDG_CACHE_HOME is not set, it returns $HOME/.cache
pub fn get_cache_home() -> Path {
    getenv_path("XDG_CACHE_HOME").unwrap_or(
        os::homedir().unwrap().join("/.cache")
    )
}

/// Get $XDG_RUNTIME_DIR if found and valid
///
/// This directory must have permissions set to 0700
pub fn get_runtime_dir() -> Option<Path> {
    getenv_path("XDG_RUNTIME_DIR")
}

/// Get an environment variable's value as a Path
fn getenv_path(env_var: &str) -> Option<Path> {
    let path = os::getenv(env_var);
    match path {
        Some(path) => Some(Path::new(path)),
        None => None
    }
}
