// pub mod bindings;
pub mod cli;
pub mod wav;
pub mod audio;

pub mod utils {
    use core::slice;

    use crate::bindings::{
        AudioQueueBufferRef, AudioQueueDispose, AudioQueueRef, AudioQueueStop, CFRunLoopGetCurrent,
        CFRunLoopStop, OSStatus,
    };

    pub fn ascii_str_transmute_u32_be(string: &str) -> Result<u32, &'static str> {
        let bytes = (*string).as_bytes();
        if bytes.len() != 4 {
            return Err("Incorrect number of bytes!");
        }
        let num_arr: [u8; 4] = bytes.try_into().unwrap();
        let num = u32::from_be_bytes(num_arr);
        Ok(num)
    }

    pub fn u32_transmute_ascii_str_le(number: u32) -> Result<String, &'static str> {
        let bytes = number.to_be_bytes();
        let result = String::from_utf8_lossy(&bytes);
        return Ok(result.to_string());
    }

    pub extern "C" fn test(
        inUserData: *mut ::std::os::raw::c_void,
        inAQ: AudioQueueRef,
        inBuffer: AudioQueueBufferRef,
    ) {
        unsafe {
            let stop_status = AudioQueueStop(inAQ, 0);
            let dispose_status = AudioQueueDispose(inAQ, 0);
            CFRunLoopStop(CFRunLoopGetCurrent());
            println!("STOP STATUS: {stop_status}\nDISPOSE_STATUS: {dispose_status}")
        }
    }

    #[repr(C)]
    pub struct TestData {
        pub num: u8,
    }
}

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(dead_code)]
pub mod bindings {
    pub const kAppleLosslessFormatFlag_16BitSourceData: u32 = 1;
    pub const kAppleLosslessFormatFlag_20BitSourceData: u32 = 2;
    pub const kAppleLosslessFormatFlag_24BitSourceData: u32 = 3;
    pub const kAppleLosslessFormatFlag_32BitSourceData: u32 = 4;
    pub const kAudioFormatFlagIsAlignedHigh: u32 = 1 << 4;
    pub const kAudioFormatFlagIsBigEndian: u32 = 1 << 1;
    pub const kAudioFormatFlagIsFloat: u32 = 1 << 0;
    pub const kAudioFormatFlagIsNonInterleaved: u32 = 1 << 5;
    pub const kAudioFormatFlagIsNonMixable: u32 = 1 << 6;
    pub const kAudioFormatFlagIsPacked: u32 = 1 << 3;
    pub const kAudioFormatFlagIsSignedInteger: u32 = 1 << 2;
    pub const kAudioFormatFlagsAreAllClear: u32 = 0x80000000;
    pub const kAudioFormatFlagsNativeEndian: u32 = 0;
    pub const kAudioFormatFlagsNativeFloatPacked: u32 =
        kAudioFormatFlagIsFloat | kAudioFormatFlagsNativeEndian | kAudioFormatFlagIsPacked;
    pub const kLinearPCMFormatFlagIsAlignedHigh: u32 = kAudioFormatFlagIsAlignedHigh;
    pub const kLinearPCMFormatFlagIsBigEndian: u32 = kAudioFormatFlagIsBigEndian;
    pub const kLinearPCMFormatFlagIsFloat: u32 = kAudioFormatFlagIsFloat;
    pub const kLinearPCMFormatFlagIsNonInterleaved: u32 = kAudioFormatFlagIsNonInterleaved;
    pub const kLinearPCMFormatFlagIsNonMixable: u32 = kAudioFormatFlagIsNonMixable;
    pub const kLinearPCMFormatFlagIsPacked: u32 = kAudioFormatFlagIsPacked;
    pub const kLinearPCMFormatFlagIsSignedInteger: u32 = kAudioFormatFlagIsSignedInteger;
    pub const kLinearPCMFormatFlagsAreAllClear: u32 = kAudioFormatFlagsAreAllClear;
    pub const kLinearPCMFormatFlagsSampleFractionShift: u32 = 7;
    pub const kLinearPCMFormatFlagsSampleFractionMask: u32 =
        0x3F << kLinearPCMFormatFlagsSampleFractionShift;

    pub const kAudioObjectSystemObject: UInt32 = 1;

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
