use std::path::Path;

use clap::Parser;
use rwav::{bindings::AudioStreamBasicDescription, cli::Cli, wav::WavHeader};

fn main() {
    println!("Hello, world!");
    let cli = Cli::parse();
    let file_path = Path::new(&(*cli.input));
    let header = WavHeader::parse(file_path).unwrap();

    let description = AudioStreamBasicDescription {
        mSampleRate: header.fmt.sample_rate as f64,
        mFormatID: rwav::utils::u32_transmute_ascii_str_le("lpcm")
            .expect("Unable to transmute!"),
        mFormatFlags: 0u32,
        mBytesPerPacket: todo!(),
        mFramesPerPacket: todo!(),
        mBytesPerFrame: todo!(),
        mChannelsPerFrame: header.fmt.num_channels as u32,
        mBitsPerChannel: header.fmt.bits_per_sample as u32,
        mReserved: todo!(),
    };
    // unsafe {
    // }
}
