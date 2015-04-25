#![cfg_attr(unix, feature(fs_ext))]

//! xdg-rs is a utility library to make conforming to the [XDG specification](http://standards.freedesktop.org/basedir-spec/basedir-spec-latest.html) easier.
//!
//! Some code borrowed from [rust-xdg](https://github.com/o11c/rust-xdg). ```rust-xdg``` is
//! currently a more complete implementation of the specification. The APIs provided by
//! ```rust-xdg``` and ```xdg-rs``` are different.

pub mod error;

pub use error::*;

use std::convert::From;
use std::env::{self, home_dir, split_paths};
use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::result;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

/// Get the data home directory given a closure that returns the the value of an environment variable.
/// This method allows having a custom environment.
///
/// If ```$XDG_DATA_HOME``` is not set, it returns ```$HOME/.local/share```.
pub fn get_data_home_from_env<F>(get_env_var: &F) -> Result<PathBuf>
    where F: Fn(&str) -> Option<OsString>
{
    get_env_path_or_default(get_env_var, "XDG_DATA_HOME", ".local/share")
}

/// Get the data home directory.
///
/// If ```$XDG_DATA_HOME``` is not set, it returns ```$HOME/.local/share```.
pub fn get_data_home() -> Result<PathBuf> {
    get_data_home_from_env(&env::var_os)
}

/// Get the default data directories given a closure that returns the the value of an environment variable.
/// This method allows having a custom environment.
///
/// If ```$XDG_DATA_DIRS``` is not set, it returns ```[/usr/local/share, /usr/share]```.
pub fn get_data_dirs_from_env<F>(get_env_var: &F) -> Vec<PathBuf>
    where F: Fn(&str) -> Option<OsString>
{
    get_env_paths_or_default(get_env_var, "XDG_DATA_DIRS", "/usr/local/share:/usr/share")
}

/// Get the data directories.
///
/// If ```$XDG_DATA_DIRS``` is not set, it returns ```[/usr/local/share, /usr/share]```.
pub fn get_data_dirs() -> Vec<PathBuf> {
    get_data_dirs_from_env(&env::var_os)
}

/// Get the config home directory given a closure that returns the the value of an environment variable.
/// This method allows having a custom environment.
///
/// If ```$XDG_CONFIG_HOME``` is not set, it returns ```$HOME/.config```.
pub fn get_config_home_from_env<F>(get_env_var: &F) -> Result<PathBuf>
    where F: Fn(&str) -> Option<OsString>
{
    get_env_path_or_default(get_env_var, "XDG_CONFIG_HOME", ".config")
}
/// Get the config home directory.
///
/// If ```$XDG_CONFIG_HOME``` is not set, it returns ```$HOME/.config```.
pub fn get_config_home() -> Result<PathBuf> {
    get_config_home_from_env(&env::var_os)
}

/// Get the default config directories given a closure that returns the the value of an environment variable.
/// This method allows having a custom environment.
///
/// If ```$XDG_CONFIG_DIRS``` is not set, it returns ```[/etc/xdg]```.
pub fn get_config_dirs_from_env<F>(get_env_var: &F) -> Vec<PathBuf>
    where F: Fn(&str) -> Option<OsString>
{
    get_env_paths_or_default(get_env_var, "XDG_CONFIG_DIRS", "/etc/xdg")
}

/// Get the config directories.
///
/// If ```$XDG_CONFIG_DIRS``` is not set, it returns ```[/etc/xdg]```.
pub fn get_config_dirs() -> Vec<PathBuf> {
    get_config_dirs_from_env(&env::var_os)
}

/// Get the cache home directory given a closure that returns the the value of an environment variable.
/// This method allows having a custom environment.
///
/// If ```$XDG_CACHE_HOME``` is not set, it returns ```$HOME/.cache```.
pub fn get_cache_home_from_env<F>(get_env_var: &F) -> Result<PathBuf>
    where F: Fn(&str) -> Option<OsString>
{
    get_env_path_or_default(get_env_var, "XDG_CACHE_HOME", ".cache")
}

/// Get the cache home directory.
///
/// If ```$XDG_CACHE_HOME``` is not set, it returns ```$HOME/.cache```.
pub fn get_cache_home() -> Result<PathBuf> {
    get_cache_home_from_env(&env::var_os)
}

/// Get $XDG_RUNTIME_DIR if found in the environment.
///
/// Returns None if ```$XDG_RUNTIME_PATH``` is not set, in which case it is up to the application.
/// to fallback to a location that conforms to the specification.
pub fn get_runtime_dir_from_env<F>(get_env_var: &F) -> Option<PathBuf>
    where F: Fn(&str) -> Option<OsString>
{
    get_env_path(get_env_var, "XDG_RUNTIME_DIR")
}

pub fn get_runtime_dir() -> Option<PathBuf> {
    get_env_path(&env::var_os, "XDG_RUNTIME_DIR")
}

/// Check that the value set for ```$XDG_RUNTIME_DIR``` meets the requirements of the specification.
///
/// Returns Ok(true) if permissions are correct, Ok(false) if permissions are incorrect, or propogates any errors that occurred while checking permissions.
///
/// >$XDG_RUNTIME_DIR defines the base directory relative to which user-specific non-essential runtime files and other file objects (such as sockets, named pipes, ...) should be stored. The directory MUST be owned by the user, and he MUST be the only one having read and write access to it. Its Unix access mode MUST be 0700.
/// >
/// >The lifetime of the directory MUST be bound to the user being logged in. It MUST be created when the user first logs in and if the user fully logs out the directory MUST be removed. If the user logs in more than once he should get pointed to the same directory, and it is mandatory that the directory continues to exist from his first login to his last logout on the system, and not removed in between. Files in the directory MUST not survive reboot or a full logout/login cycle.
/// >
/// >The directory MUST be on a local file system and not shared with any other system. The directory MUST by fully-featured by the standards of the operating system. More specifically, on Unix-like operating systems AF_UNIX sockets, symbolic links, hard links, proper permissions, file locking, sparse files, memory mapping, file change notifications, a reliable hard link count must be supported, and no restrictions on the file name character set should be imposed. Files in this directory MAY be subjected to periodic clean-up. To ensure that your files are not removed, they should have their access time timestamp modified at least once every 6 hours of monotonic time or the 'sticky' bit should be set on the file.
#[cfg(unix)]
pub fn test_runtime_dir<P: AsRef<Path>>(path: P) -> result::Result<bool, io::Error> {
    fs::metadata(path).map(|attr| (attr.permissions().mode() == 0o700))
}

#[cfg(not(unix))]
pub fn test_runtime_dir<P: AsRef<Path>>(path: P) -> result::Result<bool, io::Error> {
    Ok(true)
}

/// Get path from environment variable's value or a default path relative to home_dir
fn get_env_path_or_default<F>(get_env_var: &F, env_var: &str, default: &str) -> Result<PathBuf>
    where F: Fn(&str) -> Option<OsString>
{
    match get_env_path(get_env_var, env_var) {
        Some(p) => Ok(p),
        None => {
            match home_dir().map(|p| p.join(default)) {
                Some(p) => Ok(p),
                None => Err(From::from(XdgError::NoHomeDir)),
            }
        }
    }
}

/// Get an environment variable's value as a PathBuf.
fn get_env_path<F>(get_env_var: &F, env_var: &str) -> Option<PathBuf>
    where F: Fn(&str) -> Option<OsString>
{
    let path = (*get_env_var)(env_var);
    let path = path.map(PathBuf::from);
    match path.iter().next() {
        Some(p) if p.is_absolute() => Some(p.clone()),
        _ => None
    }
}

fn get_env_paths_or_default<F>(get_env_var: &F, env_var: &str, default: &str) -> Vec<PathBuf>
    where F: Fn(&str) -> Option<OsString>
{
    let default_paths = OsString::from(default);

    let paths = match (*get_env_var)(env_var).iter().next() {
        Some(p) if p != "" => p.clone(),
        _ => default_paths
    };

    split_paths(&paths).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashMap;
    use std::env::{self, home_dir, join_paths, split_paths};
    use std::ffi::OsString;
    use std::path::PathBuf;

    #[test]
    fn test_env_with_no_xdg_vars() {
        let mut custom_env = HashMap::new();
        custom_env.insert("dummy", "");

        let f = |var: &str| { custom_env.get(var).map(OsString::from) };

        assert!(get_data_home_from_env(&f)   == home_dir().unwrap().join(".local/share"));
        assert!(get_data_dirs_from_env(&f)   == vec![PathBuf::from("/usr/local/share"), PathBuf::from("/usr/share")]);
        assert!(get_config_home_from_env(&f) == home_dir().unwrap().join(".config"));
        assert!(get_config_dirs_from_env(&f) == vec![PathBuf::from("/etc/xdg")]);
        assert!(get_cache_home_from_env(&f)  == home_dir().unwrap().join(".cache"));
        assert!(get_runtime_dir_from_env(&f) == None);
    }

    #[test]
    fn test_env_with_empty_xdg_vars() {
        let mut custom_env = HashMap::new();
        custom_env.insert("XDG_DATA_HOME", "");
        custom_env.insert("XDG_DATA_DIRS", "");
        custom_env.insert("XDG_CONFIG_HOME", "");
        custom_env.insert("XDG_CONFIG_DIRS", "");
        custom_env.insert("XDG_CACHE_HOME", "");

        let f = |var: &str| { custom_env.get(var).map(OsString::from) };

        assert!(get_data_home_from_env(&f)   == home_dir().unwrap().join(".local/share"));
        assert!(get_data_dirs_from_env(&f)   == vec![PathBuf::from("/usr/local/share"), PathBuf::from("/usr/share")]);
        assert!(get_config_home_from_env(&f) == home_dir().unwrap().join(".config"));
        assert!(get_config_dirs_from_env(&f) == vec![PathBuf::from("/etc/xdg")]);
        assert!(get_cache_home_from_env(&f)  == home_dir().unwrap().join(".cache"));
        assert!(get_runtime_dir_from_env(&f) == None);
    }

    #[test]
    fn test_env_with_xdg_vars() {
        let cwd = PathBuf::from(&env::current_dir().unwrap());
        let mut custom_env = HashMap::new();

        custom_env.insert("XDG_DATA_HOME", cwd.join("user/data").into_os_string());
        custom_env.insert("XDG_DATA_DIRS", join_paths(vec![cwd.join("share/data"), cwd.join("local/data")]).unwrap());
        custom_env.insert("XDG_CONFIG_HOME", cwd.join("user/config").into_os_string());
        custom_env.insert("XDG_CONFIG_DIRS", join_paths(vec![cwd.join("config"), cwd.join("local/config")]).unwrap());
        custom_env.insert("XDG_CACHE_HOME", cwd.join("user/cache").into_os_string());

        let f = |var: &str| { custom_env.get(var).map(OsString::from) };

        assert!(get_data_home_from_env(&f)   == custom_env.get("XDG_DATA_HOME").map(PathBuf::from).unwrap());
        assert!(get_data_dirs_from_env(&f)   == split_paths(&custom_env["XDG_DATA_DIRS"]).collect::<Vec<PathBuf>>());
        assert!(get_config_home_from_env(&f) == custom_env.get("XDG_CONFIG_HOME").map(PathBuf::from).unwrap());
        assert!(get_config_dirs_from_env(&f) == split_paths(&custom_env["XDG_CONFIG_DIRS"]).collect::<Vec<PathBuf>>());
        assert!(get_cache_home_from_env(&f)  == custom_env.get("XDG_CACHE_HOME").map(PathBuf::from).unwrap());
    }
}
