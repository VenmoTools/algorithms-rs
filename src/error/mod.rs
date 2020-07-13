// Project algorithms-rs
// Create by VenmoSnake 2020/7/13 19:22
//

use std::fmt;

pub enum Error {
    IndexOfBounds,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::IndexOfBounds => write!(f, "Array Index of bounds!")
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::IndexOfBounds => write!(f, "Array Index of bounds!")
        }
    }
}


impl std::error::Error for Error {}