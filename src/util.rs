use std::default::Default;

use crate::{ImageError, ImageResult};

/// Create a `Vec<Pixel>` from its width and height dimensions
pub fn vec_with_dimensions<T>(width: u32, height: u32, fill: bool) -> ImageResult<Vec<T>>
where
    T: Clone + Default,
{
    if width == 0 || height == 0 {
        return Err(ImageError::InvalidDimensions(width, height));
    }

    let len = (width * height) as usize;

    Ok(if fill {
        vec![Default::default(); len]
    } else {
        Vec::with_capacity(len)
    })
}
