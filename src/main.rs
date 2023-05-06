use std::io::Write;

use fpng::{Encode, Encoder};

fn main() {
    let binding = image::open("example.png").unwrap();
    let img = binding.as_rgba8().unwrap();

    let encoded = Encoder::encode(img).unwrap();

    let mut file = std::fs::File::create("example_saved.png").unwrap();

    file.write_all(&encoded).unwrap();
}
