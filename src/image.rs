use std::io::{Read, Write};

use crate::colour::Rgb;
use crate::util::vec_with_dimensions;
use crate::{ImageError, ImageResult};

#[derive(Clone)]
pub struct Image {
    width: u32,
    height: u32,
    maxval: u8,
    values: Vec<Rgb>,
}

impl Image {
    pub fn new(width: u32, height: u32, maxval: u8) -> ImageResult<Image> {
        Ok(Image::with_values(
            width,
            height,
            maxval,
            vec_with_dimensions(width, height)?,
        ))
    }

    pub fn with_values(width: u32, height: u32, maxval: u8, values: Vec<Rgb>) -> Image {
        Image {
            width,
            height,
            maxval,
            values,
        }
    }

    pub fn from_reader<R: Read>(reader: &mut R) -> ImageResult<Image> {
        let mut magic: [u8; 1] = [0];
        reader.read_exact(&mut magic).unwrap();

        let width = 32;
        let height = 32;
        let maxval = 255;
        let values = vec_with_dimensions(width, height)?;

        Ok(Image::with_values(width, height, maxval, values))
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> ImageResult<()> {
        writeln!(
            writer,
            "P3\n{} {}\n{}",
            self.width, self.height, self.maxval
        )?;

        for (i, Rgb { r, g, b }) in self.values.iter().enumerate() {
            write!(writer, "{:03} {:03} {:03} ", r, g, b)?;

            if i % self.width as usize == 0 {
                writeln!(writer)?;
            }
        }
        Ok(())
    }

    fn index(&self, x: u32, y: u32) -> ImageResult<usize> {
        if x >= self.width && y >= self.height {
            return Err(ImageError::InvalidCoordinates(x, y));
        }

        Ok((y * self.height + x) as usize)
    }

    pub fn get(&self, x: u32, y: u32) -> ImageResult<&Rgb> {
        Ok(&self.values[self.index(x, y)?])
    }
}
