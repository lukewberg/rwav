#![feature(ascii_char)]
use std::{os::raw::c_void, path::Path, ptr};

use clap::Parser;
use rwav::{
    bindings::{
        self, kAudioFormatFlagIsPacked, kAudioFormatFlagIsSignedInteger, kCFRunLoopCommonModes,
        AudioQueueAllocateBuffer, AudioQueueBufferRef, AudioQueueEnqueueBuffer, AudioQueueRef,
        AudioQueueStart, AudioStreamBasicDescription, CFRunLoopGetCurrent, CFRunLoopRun,
    },
    cli::Cli,
    utils::{self, TestData},
    wav::{Chunk, WavFile},
};

fn main() {
    let cli = Cli::parse();
    let file_path = Path::new(&(*cli.input));
    let wav_file = WavFile::new(file_path);
    let header = wav_file.header;
    let mut data_chunk: Option<Chunk> = None;
    print!("{header:?}");

    rwav::audio::Audio::get_devices();

    wav_file.for_each(|chunk| {
        // let chunk_id: &str = chunk.chunk_header.chunk_id.as_ascii().unwrap();
        let chunk_id = String::from_utf8(chunk.chunk_header.chunk_id.to_vec()).unwrap();
        match chunk_id.as_str() {
            "info" => {
                println!("Found INFO block!");
            }
            "data" => {
                println!("Found DATA block!");
                data_chunk = Some(chunk);
            }
            _ => (),
        }
        // println!("{chunk_id:?}");
    });

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

    let test_data = TestData { num: 4 };

    let mut audio_buffer: AudioQueueBufferRef = std::ptr::null_mut();

    unsafe {
        let test = bindings::AudioQueueNewOutput(
            &description,
            Some(fn_ptr),
            std::ptr::from_ref(&test_data) as *mut c_void,
            CFRunLoopGetCurrent(),
            kCFRunLoopCommonModes,
            0,
            &mut audio_queue,
        );

        let mut chunk = data_chunk.unwrap();
        let alloc_status = AudioQueueAllocateBuffer(
            audio_queue,
            chunk.chunk_header.chunk_size,
            &mut audio_buffer,
        );
        (*audio_buffer).mAudioDataByteSize = chunk.chunk_header.chunk_size;

        let raw_data_ptr: *const c_void = chunk.data.as_ptr() as *const c_void;
        (*audio_buffer)
            .mAudioData
            .copy_from(raw_data_ptr, chunk.chunk_header.chunk_size as usize);

        let enqueue_status = AudioQueueEnqueueBuffer(audio_queue, audio_buffer, 0, ptr::null());
        let start_status = AudioQueueStart(audio_queue, ptr::null());
        CFRunLoopRun();

        if test != 0i32 {
            let error_code = utils::u32_transmute_ascii_str_le(test as u32).unwrap();
            panic!(
                "Error calling AudioToolbox framework! Returned OSStatus: {} - {}",
                error_code, test
            );
        }
        // println!("{error_code:?}");
        // println!("{:?}", *audio_queue);
        let hello = 1 + 2;
    }
}
