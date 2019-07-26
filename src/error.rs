use std::fmt;
use std::io;

pub type ImageResult<T> = Result<T, ImageError>;

/// Errors that can occur while handling an image
#[derive(Debug)]
pub enum ImageError {
    InvalidDimensions(u32, u32),
    InvalidCoordinates(u32, u32),
    IoError(io::Error),
    FormatError(String),
}

impl fmt::Display for ImageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ImageError::InvalidDimensions(w, h) => {
                write!(f, "Invalid image dimensions: {}x{}", w, h)
            }
            ImageError::InvalidCoordinates(x, y) => {
                write!(f, "Invalid image coordinates: ({}, {})", x, y)
            }
            ImageError::IoError(err) => err.fmt(f),
            ImageError::FormatError(s) => f.write_str(s),
        }
    }
}

impl From<io::Error> for ImageError {
    fn from(err: io::Error) -> ImageError {
        ImageError::IoError(err)
    }
}
