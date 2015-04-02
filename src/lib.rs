#![cfg_attr(unix, feature(old_io, old_path))]
#![feature(std_misc)]

//! xdg-rs is a utility library to make conforming to the [XDG specification](http://standards.freedesktop.org/basedir-spec/basedir-spec-latest.html) easier.
//!
//! Some code borrowed from [rust-xdg](https://github.com/o11c/rust-xdg). ```rust-xdg``` is
//! currently a more complete implementation of the specification. The APIs provided by
//! ```rust-xdg``` and ```xdg-rs``` are different.

use std::path::PathBuf;
use std::env;
use std::ffi::{self, OsStr, AsOsStr, OsString};
use std::convert::AsRef;

fn home_dir() -> PathBuf {
    PathBuf::from(env::home_dir().unwrap().as_os_str())
}

fn split_paths<P: ?Sized>(paths: &P) -> Vec<PathBuf>
    where P: AsRef<OsStr>
{
    env::split_paths(paths).map(|x| PathBuf::from(x.as_os_str())).collect()
}

/// Get the data home directory given a closure that returns the the value of an environment variable.
/// This method allows having a custom environment.
///
/// If ```$XDG_DATA_HOME``` is not set, it returns ```$HOME/.local/share```.
pub fn get_data_home_from_env<F>(get_env_var: &F) -> PathBuf where
    F: Fn(&str) -> Option<OsString>
{
    getenv_path(get_env_var, "XDG_DATA_HOME")
        .unwrap_or(home_dir().join(".local/share"))
}

/// Get the data home directory.
///
/// If ```$XDG_DATA_HOME``` is not set, it returns ```$HOME/.local/share```.
pub fn get_data_home() -> PathBuf {
    get_data_home_from_env(&env::var_os)
}

/// Get the default data directories given a closure that returns the the value of an environment variable.
/// This method allows having a custom environment.
///
/// If ```$XDG_DATA_DIRS``` is not set, it returns ```[/usr/local/share, /usr/share]```.
pub fn get_data_dirs_from_env<F>(get_env_var: &F) -> Vec<PathBuf> where
    F: Fn(&str) -> Option<OsString>
{
    let default_paths = "/usr/local/share:/usr/share".as_os_str().to_os_string();
    let paths = match (*get_env_var)("XDG_DATA_DIRS") {
        Some(paths) => {
            if paths != ffi::OsString::from("") {
                paths
            } else {
                default_paths
            }
        },
        None => default_paths
    };

    split_paths(&paths)
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
pub fn get_config_home_from_env<F>(get_env_var: &F) -> PathBuf where
    F: Fn(&str) -> Option<OsString>
{
    getenv_path(get_env_var, "XDG_CONFIG_HOME")
        .unwrap_or(home_dir().join(".config"))
}
/// Get the config home directory.
///
/// If ```$XDG_CONFIG_HOME``` is not set, it returns ```$HOME/.config```.
pub fn get_config_home() -> PathBuf {
    get_config_home_from_env(&env::var_os)
}

/// Get the default config directories given a closure that returns the the value of an environment variable.
/// This method allows having a custom environment.
///
/// If ```$XDG_CONFIG_DIRS``` is not set, it returns ```[/etc/xdg]```.
pub fn get_config_dirs_from_env<F>(get_env_var: &F) -> Vec<PathBuf> where
    F: Fn(&str) -> Option<OsString>
{
    let default_paths = "/etc/xdg".as_os_str().to_os_string();
    let paths = match (*get_env_var)("XDG_CONFIG_DIRS") {
        Some(paths) => {
            if paths != OsString::from("") {
                paths
            } else {
                default_paths
            }
        },
        None => default_paths
    };

    split_paths(&paths)
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
pub fn get_cache_home_from_env<F>(get_env_var: &F) -> PathBuf where
    F: Fn(&str) -> Option<OsString>
{
    getenv_path(get_env_var, "XDG_CACHE_HOME")
        .unwrap_or(home_dir().join(".cache"))
}

/// Get the cache home directory.
///
/// If ```$XDG_CACHE_HOME``` is not set, it returns ```$HOME/.cache```.
pub fn get_cache_home() -> PathBuf {
    get_cache_home_from_env(&env::var_os)
}

/// Get $XDG_RUNTIME_DIR if found in the environment.
///
/// Returns None if ```$XDG_RUNTIME_PATH``` is not set, in which case it is up to the application
/// to fallback to a location that conforms to the specification.
pub fn get_runtime_dir_from_env<F>(get_env_var: &F) -> Option<PathBuf> where
    F: Fn(&str) -> Option<OsString>
{
    getenv_path(get_env_var, "XDG_RUNTIME_DIR")
}

pub fn get_runtime_dir() -> Option<PathBuf> {
    getenv_path(&env::var_os, "XDG_RUNTIME_DIR")
}

/// Check that the value set for ```$XDG_RUNTIME_DIR``` meets the requirements of the specification
///
/// >$XDG_RUNTIME_DIR defines the base directory relative to which user-specific non-essential runtime files and other file objects (such as sockets, named pipes, ...) should be stored. The directory MUST be owned by the user, and he MUST be the only one having read and write access to it. Its Unix access mode MUST be 0700.
/// >
/// >The lifetime of the directory MUST be bound to the user being logged in. It MUST be created when the user first logs in and if the user fully logs out the directory MUST be removed. If the user logs in more than once he should get pointed to the same directory, and it is mandatory that the directory continues to exist from his first login to his last logout on the system, and not removed in between. Files in the directory MUST not survive reboot or a full logout/login cycle.
/// >
/// >The directory MUST be on a local file system and not shared with any other system. The directory MUST by fully-featured by the standards of the operating system. More specifically, on Unix-like operating systems AF_UNIX sockets, symbolic links, hard links, proper permissions, file locking, sparse files, memory mapping, file change notifications, a reliable hard link count must be supported, and no restrictions on the file name character set should be imposed. Files in this directory MAY be subjected to periodic clean-up. To ensure that your files are not removed, they should have their access time timestamp modified at least once every 6 hours of monotonic time or the 'sticky' bit should be set on the file.
#[cfg(unix)]
pub fn test_runtime_dir(path: &PathBuf) -> Result<(), String> {
    use std::old_path;
    use std::old_io;
    use std::os::unix::prelude::OsStrExt;
    use std::old_io::fs::PathExtensions;
    use std::error::Error;
    // FIXME: https://github.com/rust-lang/rfcs/issues/905
    match old_path::Path::new(path.as_os_str().as_bytes()).stat() {
        Ok(stat) => {
            if stat.perm.intersects(old_io::GROUP_RWX | old_io::OTHER_RWX) {
                Err("Incorrect permissions".to_string())
            } else {
                Ok(())
            }
        },

        Err(error) => Err(error.description().to_string())
    }
}

#[cfg(not(unix))]
pub fn test_runtime_dir(_path: &PathBuf) -> Result<(), String> {
    Ok(())
}


/// Get an environment variable's value as a PathBuf.
fn getenv_path<F>(get_env_var: &F, env_var: &str) -> Option<PathBuf> where
    F: Fn(&str) -> Option<OsString>
{
    let path = (*get_env_var)(env_var);
    match path {
        Some(path) => {
            let path = PathBuf::from(&path);
            if path.is_absolute() {
                Some(path)
            } else {
                None
            }
        },

        None => None
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::env;
    use std::path::PathBuf;
    use std::ffi::AsOsStr;

    #[test]
    fn test_env_with_no_xdg_vars() {
        let mut custom_env = HashMap::new();
        custom_env.insert("dummy", "");

        let f = |var: &str| { custom_env.get(var).map(|x| x.as_os_str().to_os_string()) };
        assert!(super::get_data_home_from_env(&f)
                == super::home_dir().join(".local/share"));
        assert!(super::get_data_dirs_from_env(&f)
                == vec![PathBuf::from("/usr/local/share"), PathBuf::from("/usr/share")]);
        assert!(super::get_config_home_from_env(&f)
                == super::home_dir().join(".config"));
        assert!(super::get_config_dirs_from_env(&f)
                == vec![PathBuf::from("/etc/xdg")]);
        assert!(super::get_cache_home_from_env(&f)
                == super::home_dir().join(".cache"));
        assert!(super::get_runtime_dir_from_env(&f)
                == None);
    }

    #[test]
    fn test_env_with_empty_xdg_vars() {
        let mut custom_env = HashMap::new();
        custom_env.insert("XDG_DATA_HOME", "");
        custom_env.insert("XDG_DATA_DIRS", "");
        custom_env.insert("XDG_CONFIG_HOME", "");
        custom_env.insert("XDG_CONFIG_DIRS", "");
        custom_env.insert("XDG_CACHE_HOME", "");

        let f = |var: &str| { custom_env.get(var).map(|x| x.as_os_str().to_os_string()) };
        assert!(super::get_data_home_from_env(&f)
                == super::home_dir().join(".local/share"));
        assert!(super::get_data_dirs_from_env(&f)
                == vec![PathBuf::from("/usr/local/share"), PathBuf::from("/usr/share")]);
        assert!(super::get_config_home_from_env(&f)
                == super::home_dir().join(".config"));
        assert!(super::get_config_dirs_from_env(&f)
                == vec![PathBuf::from("/etc/xdg")]);
        assert!(super::get_cache_home_from_env(&f)
                == super::home_dir().join(".cache"));
        assert!(super::get_runtime_dir_from_env(&f)
                == None);
    }

    #[test]
    fn test_env_with_xdg_vars() {
        let cwd = PathBuf::from(&env::current_dir().unwrap());
        let mut custom_env = HashMap::new();
        custom_env.insert("XDG_DATA_HOME", cwd.join("user/data").as_os_str().to_os_string());
        custom_env.insert("XDG_DATA_DIRS", env::join_paths(
                vec![cwd.join("share/data"),cwd.join("local/data")].into_iter()).unwrap());
        custom_env.insert("XDG_CONFIG_HOME", cwd.join("user/config").as_os_str().to_os_string());
        custom_env.insert("XDG_CONFIG_DIRS", env::join_paths(
                vec![cwd.join("config"), cwd.join("local/config")].into_iter()).unwrap());
        custom_env.insert("XDG_CACHE_HOME", cwd.join("user/cache").as_os_str().to_os_string());

        let f = |var: &str| { custom_env.get(var).map(|x| x.clone()) };
        assert!(super::get_data_home_from_env(&f)
                == custom_env.get("XDG_DATA_HOME").map(PathBuf::from).unwrap());
        assert!(super::get_data_dirs_from_env(&f)
                == (super::split_paths(&custom_env["XDG_DATA_DIRS"])));
        assert!(super::get_config_home_from_env(&f)
                == custom_env.get("XDG_CONFIG_HOME").map(PathBuf::from).unwrap());
        assert!(super::get_config_dirs_from_env(&f)
                == super::split_paths(&custom_env["XDG_CONFIG_DIRS"]));
        assert!(super::get_cache_home_from_env(&f)
                == custom_env.get("XDG_CACHE_HOME").map(PathBuf::from).unwrap());
    }
}
