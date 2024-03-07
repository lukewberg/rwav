use std::{os::raw::c_void, path::Path};

use clap::Parser;
use rwav::{
    bindings::{
        self, kAudioFormatFlagIsPacked, kAudioFormatFlagIsSignedInteger, kCFRunLoopDefaultMode, AudioQueueBufferRef, AudioQueueRef, AudioStreamBasicDescription, CFRunLoopGetCurrent
    },
    cli::Cli,
    utils::{self, TestData},
    wav::WavHeader,
};

fn main() {
    println!("Hello, world!");
    let cli = Cli::parse();
    let file_path = Path::new(&(*cli.input));
    let header = WavHeader::parse(file_path).unwrap();
    print!("{header:?}");

    let bytes_per_frame = ((header.fmt.num_channels * header.fmt.bits_per_sample) / 8) as u32;

    let description = AudioStreamBasicDescription {
        mSampleRate: header.fmt.sample_rate as f64,
        mFormatID: rwav::utils::ascii_str_transmute_u32_be("lpcm").expect("Unable to transmute!"),
        mFormatFlags: kAudioFormatFlagIsPacked | kAudioFormatFlagIsSignedInteger,
        mBytesPerPacket: bytes_per_frame,
        mFramesPerPacket: 1u32,
        mBytesPerFrame: bytes_per_frame,
        mChannelsPerFrame: header.fmt.num_channels as u32,
        mBitsPerChannel: header.fmt.bits_per_sample as u32,
        mReserved: 0,
    };

    let mut audio_queue: AudioQueueRef = std::ptr::null_mut(); // Create a variable to hold the AudioQueueRef

    let fn_ptr = utils::test;

    let test_data = TestData {
        num: 4
    };

    unsafe {
        let test = bindings::AudioQueueNewOutput(
            &description,
            Some(fn_ptr),
            std::ptr::from_ref(&test_data) as *mut c_void,
            CFRunLoopGetCurrent(),
            kCFRunLoopDefaultMode,
            0,
            &mut audio_queue,
        );

        if test != 0i32 {
            let error_code = utils::u32_transmute_ascii_str_le(test as u32).unwrap();
            panic!("Error calling AudioToolbox framework! Returned OSStatus: {} - {}", error_code, test);
        }
        // println!("{error_code:?}");
        // println!("{:?}", *audio_queue);
        let hello = 1 + 2;
    }
}
