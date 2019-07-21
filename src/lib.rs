use std::fs::File;
use std::io;
use std::io::Write;

const RGB_MAX: u8 = 255;

pub trait Colour: Clone {
    /// Returns the colour formatted as RGB bytes
    fn as_bytes(&self) -> (u8, u8, u8);

    /// Returns the maximum (white) value for this type
    fn white() -> Self;

    /// Returns the minimum (black) value for this type
    fn black() -> Self;
}

/// Rgb
#[derive(Debug, Clone)]
pub struct Rgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Rgb {
    /// Create a new Rgb colour
    pub fn new(r: f32, g: f32, b: f32) -> Rgb {
        Rgb { r, g, b }
    }
}

impl Colour for Rgb {
    fn as_bytes(&self) -> (u8, u8, u8) {
        (
            (self.r * RGB_MAX as f32) as u8,
            (self.g * RGB_MAX as f32) as u8,
            (self.b * RGB_MAX as f32) as u8,
        )
    }

    fn white() -> Rgb {
        Rgb {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        }
    }

    fn black() -> Rgb {
        Rgb {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }
}

/// A linear pixel array representing an image of dimensions `width`x`height`
#[derive(Debug, Clone)]
pub struct Ppm<T: Colour> {
    pixels: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Colour> Ppm<T> {
    /// Create a new PPM image with the given output dimensions
    pub fn new(width: usize, height: usize) -> Ppm<T> {
        assert!(width != 0 && height != 0);

        Ppm {
            pixels: vec![T::white(); width * height],
            width,
            height,
        }
    }

    /// Convert pixel coordinates into a 1d index
    #[inline]
    fn index(&self, x: usize, y: usize) -> Option<usize> {
        if x < self.width && y < self.height {
            return Some(y * self.width + x)
        }
        None
    }

    /// Retrieve the pixel at coordinates `(x, y)`
    #[inline]
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.index(x, y).map(|i| &self.pixels[i])
    }

    /// Retrieve a mutable reference to the pixel at coordinates `(x, y)`
    #[inline]
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        self.index(x, y).map(move |i| &mut self.pixels[i])
    }

    /// Output the image as a PPM file
    pub fn save(&self, path: &str) -> io::Result<()> {
        // Create the destination file
        let mut file = File::create(path)?;

        // Write PPM header for RGB with dimensions and max pixel value
        writeln!(file, "P3")?;
        writeln!(file, "{} {}", self.height, self.width)?;
        writeln!(file, "{}", RGB_MAX)?;

        // Write the each pixel row by row
        for y in 0..self.height {
            for x in 0..self.width {
                let (r, g, b) = self.get(x, y).unwrap().as_bytes();
                write!(file, "{:03} {:03} {:03}  ", r, g, b)?;
            }
            writeln!(file, "")?;
        }

        Ok(())
    }
}
