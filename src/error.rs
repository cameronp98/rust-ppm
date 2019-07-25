use std::fmt;
use std::io;

pub type PpmResult<T> = Result<T, PpmError>;

/// Errors that may occur while performing operations on a PPM image
#[derive(Debug)]
pub enum PpmError {
    InvalidDimensions(usize, usize),
    NotEnoughPixels(usize, usize),
    Io(io::Error),
}

impl fmt::Display for PpmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PpmError::InvalidDimensions(w, h) => {
                write!(f, "Invalid image dimensions: {}x{}", w, h)
            },
            PpmError::NotEnoughPixels(exp, got) => {
                write!(f, "Expected {} pixels, got {}", exp, got)
            },
            PpmError::Io(err) => err.fmt(f),
        }
    }
}

impl From<io::Error> for PpmError {
    fn from(err: io::Error) -> PpmError {
        PpmError::Io(err)
    }
}
