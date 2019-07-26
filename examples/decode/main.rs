extern crate ppm;

use std::fs::File;

use ppm::Image;

fn main() {
    let mut file = File::open("examples/decode/image.ppm").unwrap();
    let image = Image::from_reader(&mut file).unwrap();
    println!("{:?}", image.values());
}
