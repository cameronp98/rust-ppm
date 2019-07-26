use std::fmt;
use std::io;
use std::num::ParseIntError;

pub type ImageResult<T> = Result<T, ImageError>;

/// Errors that can occur while handling an image
#[derive(Debug)]
pub enum ImageError {
    InvalidDimensions(u32, u32),
    InvalidLocation(u32, u32),
    UnexpectedEof,
    FormatError(String),
    IoError(io::Error),
    ParseIntError(ParseIntError),
}

impl fmt::Display for ImageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ImageError::InvalidDimensions(w, h) => {
                write!(f, "Invalid image dimensions: {}x{}", w, h)
            }
            ImageError::InvalidLocation(x, y) => {
                write!(f, "Invalid pixel location: ({}, {})", x, y)
            }
            ImageError::UnexpectedEof => write!(f, "Unexpected EOF"),
            ImageError::FormatError(s) => write!(f, "Format error: {}", s),
            ImageError::IoError(e) => e.fmt(f),
            ImageError::ParseIntError(e) => e.fmt(f),
        }
    }
}

impl From<io::Error> for ImageError {
    fn from(err: io::Error) -> ImageError {
        ImageError::IoError(err)
    }
}

impl From<ParseIntError> for ImageError {
    fn from(err: ParseIntError) -> ImageError {
        ImageError::ParseIntError(err)
    }
}
