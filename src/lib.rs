#![feature(os, core, old_io, old_path, env)]

//! xdg-rs is a utility library to make conforming to the [XDG specification](http://standards.freedesktop.org/basedir-spec/basedir-spec-latest.html) easier.
//!
//! Some code borrowed from [rust-xdg](https://github.com/o11c/rust-xdg). ```rust-xdg``` is
//! currently a more complete implementation of the specification. The APIs provided by
//! ```rust-xdg``` and ```xdg-rs``` are different.

use std::error::Error;
use std::old_io as io;
use std::old_io::fs::PathExtensions;
use std::os;
use std::env;

/// Get the data home directory given a closure that returns the the value of an environment variable.
/// This method allows having a custom environment.
///
/// If ```$XDG_DATA_HOME``` is not set, it returns ```$HOME/.local/share```.
pub fn get_data_home_from_env<F>(get_env_var: &F) -> Path where
    F: Fn(&str) -> Option<String>
{
    getenv_path(get_env_var, "XDG_DATA_HOME")
        .unwrap_or(env::home_dir()
                       .unwrap()
                       .join(".local/share"))
}

/// Get the data home directory.
///
/// If ```$XDG_DATA_HOME``` is not set, it returns ```$HOME/.local/share```.
pub fn get_data_home() -> Path {
    get_data_home_from_env(&os::getenv)
}

/// Get the default data directories given a closure that returns the the value of an environment variable.
/// This method allows having a custom environment.
///
/// If ```$XDG_DATA_DIRS``` is not set, it returns ```[/usr/local/share, /usr/share]```.
pub fn get_data_dirs_from_env<F>(get_env_var: &F) -> Vec<Path> where
    F: Fn(&str) -> Option<String>
{
    let default_paths = "/usr/local/share:/usr/share".to_string();
    let paths = match (*get_env_var)("XDG_DATA_DIRS") {
        Some(paths) => {
            if paths.len() > 0 {
                paths
            } else {
                default_paths
            }
        },
        None => default_paths
    };

    env::split_paths(&paths).collect()
}

/// Get the data directories.
///
/// If ```$XDG_DATA_DIRS``` is not set, it returns ```[/usr/local/share, /usr/share]```.
pub fn get_data_dirs() -> Vec<Path> {
    get_data_dirs_from_env(&os::getenv)
}

/// Get the config home directory given a closure that returns the the value of an environment variable.
/// This method allows having a custom environment.
///
/// If ```$XDG_CONFIG_HOME``` is not set, it returns ```$HOME/.config```.
pub fn get_config_home_from_env<F>(get_env_var: &F) -> Path where
    F: Fn(&str) -> Option<String>
{
    getenv_path(get_env_var, "XDG_CONFIG_HOME")
        .unwrap_or(env::home_dir()
                       .unwrap()
                       .join(".config"))
}
/// Get the config home directory.
///
/// If ```$XDG_CONFIG_HOME``` is not set, it returns ```$HOME/.config```.
pub fn get_config_home() -> Path {
    get_config_home_from_env(&os::getenv)
}

/// Get the default config directories given a closure that returns the the value of an environment variable.
/// This method allows having a custom environment.
///
/// If ```$XDG_CONFIG_DIRS``` is not set, it returns ```[/etc/xdg]```.
pub fn get_config_dirs_from_env<F>(get_env_var: &F) -> Vec<Path> where
    F: Fn(&str) -> Option<String>
{
    let default_paths = "/etc/xdg".to_string();
    let paths = match (*get_env_var)("XDG_CONFIG_DIRS") {
        Some(paths) => {
            if paths.len() > 0 {
                paths
            } else {
                default_paths
            }
        },
        None => default_paths
    };

    env::split_paths(&paths).collect()
}

/// Get the config directories.
///
/// If ```$XDG_CONFIG_DIRS``` is not set, it returns ```[/etc/xdg]```.
pub fn get_config_dirs() -> Vec<Path> {
    get_config_dirs_from_env(&os::getenv)
}

/// Get the cache home directory given a closure that returns the the value of an environment variable.
/// This method allows having a custom environment.
///
/// If ```$XDG_CACHE_HOME``` is not set, it returns ```$HOME/.cache```.
pub fn get_cache_home_from_env<F>(get_env_var: &F) -> Path where
    F: Fn(&str) -> Option<String>
{
    getenv_path(get_env_var, "XDG_CACHE_HOME")
        .unwrap_or(env::home_dir()
                       .unwrap()
                       .join(".cache"))
}

/// Get the cache home directory.
///
/// If ```$XDG_CACHE_HOME``` is not set, it returns ```$HOME/.cache```.
pub fn get_cache_home() -> Path {
    get_cache_home_from_env(&os::getenv)
}

/// Get $XDG_RUNTIME_DIR if found in the environment.
///
/// Returns None if ```$XDG_RUNTIME_PATH``` is not set, in which case it is up to the application
/// to fallback to a location that conforms to the specification.
pub fn get_runtime_dir_from_env<F>(get_env_var: &F) -> Option<Path> where
    F: Fn(&str) -> Option<String>
{
    getenv_path(get_env_var, "XDG_RUNTIME_DIR")
}

pub fn get_runtime_dir() -> Option<Path> {
    getenv_path(&os::getenv, "XDG_RUNTIME_DIR")
}

/// Check that the value set for ```$XDG_RUNTIME_DIR``` meets the requirements of the specification
///
/// >$XDG_RUNTIME_DIR defines the base directory relative to which user-specific non-essential runtime files and other file objects (such as sockets, named pipes, ...) should be stored. The directory MUST be owned by the user, and he MUST be the only one having read and write access to it. Its Unix access mode MUST be 0700.
/// >
/// >The lifetime of the directory MUST be bound to the user being logged in. It MUST be created when the user first logs in and if the user fully logs out the directory MUST be removed. If the user logs in more than once he should get pointed to the same directory, and it is mandatory that the directory continues to exist from his first login to his last logout on the system, and not removed in between. Files in the directory MUST not survive reboot or a full logout/login cycle.
/// >
/// >The directory MUST be on a local file system and not shared with any other system. The directory MUST by fully-featured by the standards of the operating system. More specifically, on Unix-like operating systems AF_UNIX sockets, symbolic links, hard links, proper permissions, file locking, sparse files, memory mapping, file change notifications, a reliable hard link count must be supported, and no restrictions on the file name character set should be imposed. Files in this directory MAY be subjected to periodic clean-up. To ensure that your files are not removed, they should have their access time timestamp modified at least once every 6 hours of monotonic time or the 'sticky' bit should be set on the file.
pub fn test_runtime_dir(path: &Path) -> Result<(), String> {
    match path.stat() {
        Ok(stat) => {
            if stat.perm.intersects(io::GROUP_RWX | io::OTHER_RWX) {
                Err("Incorrect permissions".to_string())
            } else {
                Ok(())
            }
        },

        Err(error) => Err(error.description().to_string())
    }
}

/// Get an environment variable's value as a Path.
fn getenv_path<F>(get_env_var: &F, env_var: &str) -> Option<Path> where
    F: Fn(&str) -> Option<String>
{
    let path = (*get_env_var)(env_var);
    match path {
        Some(path) => {
            let path = Path::new(path);
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
    use std::os;

    #[test]
    fn test_env_with_no_xdg_vars() {
        let mut custom_env = HashMap::new();
        custom_env.insert("dummy", "".to_string());

        let f = |&: var: &str| { custom_env.get(var).map(|x| x.clone()) };
        assert!(super::get_data_home_from_env(&f)
                == env::home_dir().unwrap().join(".local/share"));
        assert!(super::get_data_dirs_from_env(&f)
                == vec![Path::new("/usr/local/share"), Path::new("/usr/share")]);
        assert!(super::get_config_home_from_env(&f)
                == env::home_dir().unwrap().join(".config"));
        assert!(super::get_config_dirs_from_env(&f)
                == vec![Path::new("/etc/xdg")]);
        assert!(super::get_cache_home_from_env(&f)
                == env::home_dir().unwrap().join(".cache"));
        assert!(super::get_runtime_dir_from_env(&f)
                == None);
    }

    #[test]
    fn test_env_with_empty_xdg_vars() {
        let mut custom_env = HashMap::new();
        custom_env.insert("XDG_DATA_HOME", "".to_string());
        custom_env.insert("XDG_DATA_DIRS", "".to_string());
        custom_env.insert("XDG_CONFIG_HOME", "".to_string());
        custom_env.insert("XDG_CONFIG_DIRS", "".to_string());
        custom_env.insert("XDG_CACHE_HOME", "".to_string());

        let f = |&: var: &str| { custom_env.get(var).map(|x| x.clone()) };
        assert!(super::get_data_home_from_env(&f)
                == env::home_dir().unwrap().join(".local/share"));
        assert!(super::get_data_dirs_from_env(&f)
                == vec![Path::new("/usr/local/share"), Path::new("/usr/share")]);
        assert!(super::get_config_home_from_env(&f)
                == env::home_dir().unwrap().join(".config"));
        assert!(super::get_config_dirs_from_env(&f)
                == vec![Path::new("/etc/xdg")]);
        assert!(super::get_cache_home_from_env(&f)
                == env::home_dir().unwrap().join(".cache"));
        assert!(super::get_runtime_dir_from_env(&f)
                == None);
    }

    #[test]
    fn test_env_with_xdg_vars() {
        let cwd = os::make_absolute(&Path::new(".")).unwrap();
        let mut custom_env = HashMap::new();
        custom_env.insert("XDG_DATA_HOME", format!("{}/user/data", cwd.display()));
        custom_env.insert("XDG_DATA_DIRS", format!("{}/share/data:{}/local/data", cwd.display(), cwd.display()));
        custom_env.insert("XDG_CONFIG_HOME", format!("{}/user/config", cwd.display()));
        custom_env.insert("XDG_CONFIG_DIRS", format!("{}/config:{}/local/config", cwd.display(), cwd.display()));
        custom_env.insert("XDG_CACHE_HOME", format!("{}/user/cache", cwd.display()));

        let f = |&: var: &str| { custom_env.get(var).map(|x| x.clone()) };
        assert!(super::get_data_home_from_env(&f)
                == custom_env.get("XDG_DATA_HOME").map(Path::new).unwrap());
        assert!(super::get_data_dirs_from_env(&f)
                == (env::split_paths(&*custom_env["XDG_DATA_DIRS"]).collect::<Vec<Path>>()));
        assert!(super::get_config_home_from_env(&f)
                == custom_env.get("XDG_CONFIG_HOME").map(Path::new).unwrap());
        assert!(super::get_config_dirs_from_env(&f)
                == env::split_paths(&*custom_env["XDG_CONFIG_DIRS"]).collect::<Vec<Path>>());
        assert!(super::get_cache_home_from_env(&f)
                == custom_env.get("XDG_CACHE_HOME").map(Path::new).unwrap());
    }
}
