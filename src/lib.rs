use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::Path;

pub mod colour;
pub mod error;
pub mod util;

use error::{PpmError, PpmResult};
use util::RGB_MAX;

use colour::Rgb;

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
    /// # Arguments
    ///
    /// * `width` - The width of the image in pixels
    /// * `height` - The height of the image in pixels
    ///
    /// # Examples
    ///
    /// ```
    /// use ppm::Ppm;
    ///
    /// let ppm = Ppm::new(32, 32);
    /// ppm.save("image.ppm");
    /// ```
    pub fn new(width: usize, height: usize) -> PpmResult<Ppm> {
        Ppm::with_pixels(width, height, vec![Rgb::black(); width * height])
    }

    /// Create a new PPM image with the given pixel values
    pub fn with_pixels(width: usize, height: usize, pixels: Vec<Rgb>) -> PpmResult<Ppm> {
        if width == 0 || height == 0 || width * height < pixels.len() {
            return Err(PpmError::InvalidDimensions(width, height));
        }

        Ok(Ppm {
            width,
            height,
            pixels,
        })
    }

    /// Read a PPM image from a file
    ///
    /// # Examples
    ///
    /// ```
    /// use ppm::Ppm;
    ///
    /// let ppm = Ppm::from_file("image.ppm");
    /// *ppm.get_mut(0, 0).unwrap() = Rgb::black();
    /// ppm.save();
    /// ```
    pub fn from_file<P: AsRef<Path>>(path: P) -> PpmResult<Ppm> {
        let mut file = File::open(path)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;

        let (width, height) = (1, 1);
        let mut pixels = util::create_pixels(width, height, Rgb::black())?;

        // @TODO: parse and create the image here

        Ppm::with_pixels(width, height, pixels)
    }

    /// Convert pixel coordinates into a 1d index
    ///
    /// # Arguments
    ///
    /// * `x` - The x coordinate of the pixel (its column index)
    /// * `y` - The y coordinate of the pixel (its row index)
    #[inline]
    fn index(&self, x: usize, y: usize) -> Option<usize> {
        if x < self.width && y < self.height {
            return Some(y * self.width + x);
        }
        None
    }

    /// Retrieve the pixel at coordinates (`x`, `y`)
    ///
    /// # Arguments
    ///
    /// * `x` - The x coordinate of the pixel (its column index)
    /// * `y` - The y coordinate of the pixel (its row index)
    ///
    /// # Examples
    ///
    /// ```
    /// use ppm::Ppm;
    ///
    /// let ppm = Ppm::new(32, 32);
    /// assert_eq!(ppm.get(0, 0).unwrap().r, 0.0);
    /// ```
    #[inline]
    pub fn get(&self, x: usize, y: usize) -> Option<&Rgb> {
        self.index(x, y).map(|i| &self.pixels[i])
    }

    /// Retrieve a mutable reference to the pixel at coordinates (`x`, `y`)
    /// # Arguments
    ///
    /// * `x` - The x coordinate of the pixel (its column index)
    /// * `y` - The y coordinate of the pixel (its row index)
    ///
    /// # Examples
    ///
    /// ```
    /// use ppm::{Ppm, Rgb};
    ///
    /// let mut ppm = Ppm::new(32, 32);
    /// // Change a whole pixel
    /// if let Some(pixel) = ppm.get_mut(0, 0) {
    ///     *pixel = Rgb::new(0.3, 1.0, 1.0);
    /// }
    /// // Change the red component individually
    /// ppm.get_mut(0, 0).unwrap().r += 0.1;
    ///
    /// assert_eq!(ppm.get(0, 0).unwrap().r, 0.4);
    /// ```
    #[inline]
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Rgb> {
        self.index(x, y).map(move |i| &mut self.pixels[i])
    }

    /// Output the image as a PPM file
    ///
    /// # Arguments
    ///
    /// * `path` - Where to save the image
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
            writeln!(file)?;
        }

        Ok(())
    }
}
