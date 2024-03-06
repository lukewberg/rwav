// pub mod bindings;
pub mod cli;
pub mod wav;

pub mod utils {
    use core::slice;

    use crate::bindings::{AudioQueueBufferRef, AudioQueueRef};

    pub fn ascii_transmute_u32_str_be(string: &str) -> Result<u32, &'static str> {
        let bytes = (*string).as_bytes();
        if bytes.len() != 4 {
            return Err("Incorect number of bytes!");
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
        println!("{inAQ:?}");
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
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
