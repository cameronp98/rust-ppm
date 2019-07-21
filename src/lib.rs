use std::fs::File;
use std::io;
use std::io::Write;

pub const RGB_MAX: u8 = 255;

/// Convert RGB f32 [0.0, 1.0] to u8 [0, 255]
pub fn f32_to_u8(f: f32) -> u8 {
    assert!(f <= 1.0);
    (f * RGB_MAX as f32) as u8
}

/// Convert RGB u8 [0, 255] to f32 [0.0, 1.0]
pub fn u8_to_f32(u: u8) -> f32 {
    u as f32 / RGB_MAX as f32
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

    fn as_bytes(&self) -> (u8, u8, u8) {
        (
            f32_to_u8(self.r),
            f32_to_u8(self.g),
            f32_to_u8(self.b),
        )
    }

    pub fn white() -> Rgb {
        Rgb {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        }
    }

    pub fn black() -> Rgb {
        Rgb {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }
}

/// A linear pixel array representing an image of dimensions `width`x`height`
#[derive(Debug, Clone)]
pub struct Ppm {
    pixels: Vec<Rgb>,
    width: usize,
    height: usize,
}

impl Ppm {
    /// Create a new PPM image with the given output dimensions
    pub fn new(width: usize, height: usize) -> Ppm {
        assert!(width != 0 && height != 0);

        Ppm {
            pixels: vec![Rgb::black(); width * height],
            width,
            height,
        }
    }

    /// Convert pixel coordinates into a 1d index
    #[inline]
    pub fn index(&self, x: usize, y: usize) -> Option<usize> {
        if x < self.width && y < self.height {
            return Some(y * self.width + x)
        }
        None
    }

    /// Retrieve the pixel at coordinates `(x, y)`
    #[inline]
    pub fn get(&self, x: usize, y: usize) -> Option<&Rgb> {
        self.index(x, y).map(|i| &self.pixels[i])
    }

    /// Retrieve a mutable reference to the pixel at coordinates `(x, y)`
    #[inline]
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Rgb> {
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
