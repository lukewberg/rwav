use std::path::Path;

use rwav::wav::WavHeader;

fn main() {
    println!("Hello, world!");
    let file_path = Path::new(r"/Users/lukeberg/Downloads/Overture.wav");
    WavHeader::parse(file_path);
}
