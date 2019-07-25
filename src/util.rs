use crate::{Rgb, PpmResult, PpmError};

/// The maximum value for a single RGB component
pub const RGB_MAX: u8 = 255;

/// Convert RGB `f32` in [0.0, 1.0] to `u8` in [0, 255]
///
/// # Examples
///
/// ```
/// use ppm::{Ppm, Rgb, float_to_byte};
///
/// assert_eq!(float_to_byte(1.0), 255);
/// ```
pub fn float_to_byte(f: f32) -> u8 {
    assert!(f <= 1.0);
    (f * f32::from(RGB_MAX)) as u8
}

/// Convert RGB `u8` in [0, 255] to `f32` in [0.0, 1.0]
///
/// # Examples
///
/// ```
/// use ppm::byte_to_float;
/// assert_eq!(byte_to_float(255), 1.0);
/// ```
pub fn byte_to_float(u: u8) -> f32 {
    u as f32 / f32::from(RGB_MAX)
}

/// Create a `Vec<Pixel>` from its width and height dimensions
pub fn create_pixels(width: usize, height: usize, colour: Rgb) -> PpmResult<Vec<Rgb>> {
    if width == 0 || height == 0 {
        return Err(PpmError::InvalidDimensions(width, height))
    }

    Ok(vec![colour; width * height])
}
