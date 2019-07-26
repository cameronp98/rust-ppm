use std::fs::File;
use std::path::Path;

mod error;
mod image;
mod util;

pub use error::{ImageError, ImageResult};
pub use image::{Image, Rgb};

/// Decode an image file and return the image
pub fn load<P: AsRef<Path>>(path: P) -> ImageResult<Image> {
    Image::from_reader(&mut File::open(path)?)
}
