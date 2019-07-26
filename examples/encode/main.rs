use ppm::load;
use std::io::stdout;

fn main() {
    let image = load("examples/encode/image.ppm").unwrap();
    image.write_ascii(&mut stdout().lock()).unwrap();
}
