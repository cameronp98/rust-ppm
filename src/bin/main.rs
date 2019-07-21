use ppm::{Ppm, Rgb};

fn string_to_ppm(s: &str) -> Ppm {
    let bytes = s.as_bytes();

    let size = (bytes.len() as f32).sqrt().ceil() as usize;
    let mut ppm = Ppm::new(size, size);

    for x in 0..size {
        for y in 0..size {
            if let Some(pixel) = ppm.get_mut(x, y) {
                
            }
        }
    }

    ppm
}

fn main() {
    string_to_ppm("cameron").save("image.ppm");
}
