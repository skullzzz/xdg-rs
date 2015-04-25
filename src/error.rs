use std::convert::From;
use std::error;
use std::fmt;
use std::io;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    error_kind: ErrorKind,
}

#[derive(Debug)]
pub enum ErrorKind {
    Xdg(XdgError),
    Io(io::Error)
}

#[derive(Debug)]
pub enum XdgError {
    NoHomeDir,
    IncorrectPermissions,
    IncorrectOwner,
}

impl Error {
    pub fn new(error_kind: ErrorKind) -> Error {
        Error { error_kind: error_kind }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.error_kind {
            ErrorKind::Xdg(ref e) => write!(f, "Xdg error: {:?}", e),
            ErrorKind::Io(ref e) => e.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self.error_kind {
            ErrorKind::Xdg(_) => "Xdg error",
            ErrorKind::Io(ref e) => e.description(),
        }
    }
}

impl From<XdgError> for Error {
    fn from(error: XdgError) -> Error {
        Error { error_kind: ErrorKind::Xdg(error) }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error { error_kind: ErrorKind::Io(error) }
    }
}
