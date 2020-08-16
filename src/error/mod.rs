// Project algorithms-rs
// Create by VenmoSnake 2020/7/13 19:22
//

use std::fmt;
use std::fmt::{Debug, Display};
use std::io::ErrorKind;
use std::num::ParseIntError;

pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    IndexOfBounds,
    ParallelEdges,
    SelfLoop,
    IOError(std::io::Error),
    ConvertError(std::num::ParseIntError),
    UnKnownError(Box<dyn std::error::Error>),
}

impl Error {
    pub fn from_io(kind: ErrorKind) -> Self {
        Error::IOError(std::io::Error::from(kind))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::IndexOfBounds => write!(f, "Array Index of bounds!"),
            Error::IOError(ref e) => write!(f, "{}", e),
            Error::ConvertError(ref e) => write!(f, "{}", e),
            Error::SelfLoop => write!(f, "Self loop is Detected"),
            Error::ParallelEdges => write!(f, "Parallel Edged are Detected"),
            Error::UnKnownError(ref e) => write!(f, "{}", e),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}


impl std::error::Error for Error {}


impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IOError(e)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Error::ConvertError(e)
    }
}

impl From<Box<dyn std::error::Error>> for Error {
    fn from(e: Box<dyn std::error::Error>) -> Self {
        Error::UnKnownError(e)
    }
}