use ppm::{Ppm, Rgb};

fn main() {
    let width = 64;
    let height = width;

    let mut ppm = Ppm::new(width, height);

    for x in 0..width {
        for y in 0..height {
            if let Some(pixel) = ppm.get_mut(x, y) {
                *pixel = Rgb::new(
                    x as f32 / width as f32,
                    y as f32 / height as f32,
                    0.5);
            }
        }
    }

    ppm.save("image.ppm").unwrap();
}
