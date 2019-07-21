use std::fs::File;
use std::io;
use std::io::Write;

pub const RGB_MAX: u8 = 255;

/// Convert an RGB f32 in [0.0, 1.0] to an u8 in [0, 255]
pub fn f32_to_u8(f: f32) -> u8 {
    assert!(f <= 1.0);
    (f * RGB_MAX as f32) as u8
}

/// Convert an RGB u8 in [0, 255] to an f32 in [0.0, 1.0]
pub fn u8_to_f32(u: u8) -> f32 {
    u as f32 / RGB_MAX as f32
}

/// An RGB pixel with values in [0.0, 1.0]
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

/// A PPM image encoded as a linear pixel array
/// 
/// # Examples
/// 
/// ```
/// use ppm::{Ppm, Rgb};
/// 
/// // Create a blank 32x32 image
/// let mut img = Ppm::new(32, 32);
/// // Change a pixel
/// *img.get_mut(5, 7).unwrap() = Rgb::new(1.0, 1.0, 1.0);
/// // Save the image
/// img.save("image.ppm").unwrap();
/// 
#[derive(Debug, Clone)]
pub struct Ppm {
    /// The image pixels stored contiguously
    pixels: Vec<Rgb>,
    /// The image width in pixels (or the number of rows)
    width: usize,
    /// The image height in pixels (or the number of columns)
    height: usize,
}

impl Ppm {
    /// Create a new PPM image with a width and height
    /// 
    /// # Examples
    /// 
    /// ```
    /// use ppm::Ppm;
    /// 
    /// let ppm = Ppm::new(32, 32);
    /// ppm.save("image.ppm");
    /// ```
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
    fn index(&self, x: usize, y: usize) -> Option<usize> {
        if x < self.width && y < self.height {
            return Some(y * self.width + x)
        }
        None
    }

    /// Retrieve the pixel at coordinates (`x`, `y`)
    /// 
    /// # Examples
    /// 
    /// ```
    /// use ppm::Ppm;
    /// 
    /// let ppm = Ppm::new(32, 32);
    /// println!("{:?}", ppm.get(20, 20).unwrap());
    /// ```
    #[inline]
    pub fn get(&self, x: usize, y: usize) -> Option<&Rgb> {
        self.index(x, y).map(|i| &self.pixels[i])
    }

    /// Retrieve a mutable reference to the pixel at coordinates (`x`, `y`)
    /// 
    /// # Examples
    /// 
    /// ```
    /// use ppm::{Ppm, Rgb};
    /// 
    /// let mut ppm = Ppm::new(32, 32);
    /// // Change a whole pixel
    /// if let Some(pixel) = ppm.get_mut(20, 20) {
    ///     *pixel = Rgb::new(0.3, 1.0, 0.6); 
    /// }
    /// // Change the red component individually
    /// ppm.get_mut(20, 20).unwrap().r += 0.1;
    /// ```
    #[inline]
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Rgb> {
        self.index(x, y).map(move |i| &mut self.pixels[i])
    }

    /// Output the image as a PPM file
    /// 
    /// # Examples
    /// 
    /// ```
    /// use ppm::Ppm;
    /// 
    /// let ppm = Ppm::new(32, 32);
    /// ppm.save("image.ppm");
    /// ```
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
