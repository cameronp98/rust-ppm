use crate::util::float_to_byte;

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

    pub fn as_bytes(&self) -> (u8, u8, u8) {
        (
            float_to_byte(self.r),
            float_to_byte(self.g),
            float_to_byte(self.b),
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
