use std::path::Path;

use clap::Parser;
use rwav::{cli::Cli, wav::WavHeader};

fn main() {
    println!("Hello, world!");
    let cli = Cli::parse();
    let file_path = Path::new(&(*cli.input));
    WavHeader::parse(file_path);
}
