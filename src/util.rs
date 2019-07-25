/// The maximum value for a single RGB component
pub const RGB_MAX: u8 = 255;

/// Convert RGB `f32` in [0.0, 1.0] to `u8` in [0, 255]
/// 
/// # Examples
/// 
/// ```
/// use ppm::{Ppm, Rgb, f32_to_u8};
/// 
/// assert_eq!(f32_to_u8(1.0), 255);
/// ```
pub fn f32_to_u8(f: f32) -> u8 {
    assert!(f <= 1.0);
    (f * RGB_MAX as f32) as u8
}

/// Convert RGB `u8` in [0, 255] to `f32` in [0.0, 1.0]
/// 
/// # Examples
/// 
/// ```
/// use ppm::u8_to_f32;
/// assert_eq!(u8_to_f32(255), 1.0);
/// ```
pub fn u8_to_f32(u: u8) -> f32 {
    u as f32 / RGB_MAX as f32
}
