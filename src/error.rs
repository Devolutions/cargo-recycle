
use std::io;
use std::fmt;
use std::error;

#[derive(Debug)]
#[allow(dead_code)]
pub enum RecycleError {
    Io(io::Error),
}

impl fmt::Display for RecycleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RecycleError::Io(ref err) => write!(f, "IO error: {}", err),
        }
    }
}

impl error::Error for RecycleError {
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            RecycleError::Io(ref err) => Some(err),
        }
    }
}
