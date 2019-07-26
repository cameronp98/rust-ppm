use std::io::{Read, Write};
use std::num::ParseIntError;
use std::str::FromStr;

use crate::util::vec_with_dimensions;
use crate::{ImageError, ImageResult};

/// An RGB colour
#[derive(Clone, Default, Debug)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub fn new(r: u8, g: u8, b: u8) -> Rgb {
        Rgb { r, g, b }
    }

    pub fn write_raw<W: Write>(&self, writer: &mut W) -> ImageResult<()> {
        writer.write_all(&[self.r, self.g, self.b])?;
        Ok(())
    }

    pub fn write_ascii<W: Write>(&self, writer: &mut W) -> ImageResult<()> {
        write!(writer, "{:3} {:3} {:3}", self.r, self.g, self.b)?;
        Ok(())
    }
}

/// A Netpbm image
#[derive(Clone)]
pub struct Image {
    width: u32,
    height: u32,
    maxval: u8,
    values: Vec<Rgb>,
}

impl Image {
    /// Create a new image with the given dimensions and maximum colour value
    ///
    /// Width and height must be non-zero as checked by `vec_with_dimensions()`
    pub fn new(width: u32, height: u32, maxval: u8) -> ImageResult<Image> {
        Ok(Image::with_values(
            width,
            height,
            maxval,
            vec_with_dimensions(width, height, true)?,
        ))
    }

    /// Create a new image with predefined colour values
    pub fn with_values(width: u32, height: u32, maxval: u8, values: Vec<Rgb>) -> Image {
        Image {
            width,
            height,
            maxval,
            values,
        }
    }

    /// Create an image from the stream `reader`
    pub fn from_reader<R: Read>(reader: &mut R) -> ImageResult<Image> {
        let mut magic: [u8; 2] = [0, 0];
        reader.read_exact(&mut magic)?;

        if magic[0] != b'P' || magic[1] != b'3' {
            return Err(ImageError::FormatError(format!(
                "Invalid magic number {:?}",
                magic
            )));
        }

        let mut buf = String::new();
        reader.read_to_string(&mut buf)?;

        let mut words = buf.split_ascii_whitespace();

        let width: u32 = words.parse_next()?;
        let height: u32 = words.parse_next()?;
        let maxval: u8 = words.parse_next()?;

        let mut pixels = vec_with_dimensions(width, height, false)?;

        for _i in 0..(width * height) {
            pixels.push(Rgb {
                r: words.parse_next()?,
                g: words.parse_next()?,
                b: words.parse_next()?,
            });
        }

        Ok(Image::with_values(width, height, maxval, pixels))
    }

    /// Encode the image and write it to the stream `writer` in ASCII format
    pub fn write_ascii<W: Write>(&self, writer: &mut W) -> ImageResult<()> {
        writeln!(
            writer,
            "P3\n{} {}\n{}",
            self.width, self.height, self.maxval
        )?;

        for y in 0..self.width {
            for x in 0..self.height {
                self.get(x, y).unwrap().write_ascii(writer)?;
                write!(writer, " ")?;
            }
            writeln!(writer)?;
        }

        Ok(())
    }

    // Convert the position of a pixel in the image into its pixel array index
    fn index(&self, x: u32, y: u32) -> ImageResult<usize> {
        if x >= self.width && y >= self.height {
            return Err(ImageError::InvalidLocation(x, y));
        }

        Ok((y * self.height + x) as usize)
    }

    /// Retrieve the value of the pixel at position (`x`, `y`)
    pub fn get(&self, x: u32, y: u32) -> ImageResult<&Rgb> {
        Ok(&self.values[self.index(x, y)?])
    }

    /// Retrieve a reference to the image's pixel array
    pub fn values(&self) -> &Vec<Rgb> {
        &self.values
    }
}

trait ParseNext {
    fn parse_next<F: FromStr<Err = ParseIntError>>(&mut self) -> ImageResult<F>;
}

impl<'a, I: Iterator<Item = &'a str>> ParseNext for I {
    fn parse_next<F: FromStr<Err = ParseIntError>>(&mut self) -> ImageResult<F> {
        let ret = self.next().ok_or(ImageError::UnexpectedEof)?.parse()?;
        Ok(ret)
    }
}
