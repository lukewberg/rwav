// pub mod bindings;
pub mod cli;
pub mod wav;

pub mod utils {
    use crate::bindings::{AudioQueueBufferRef, AudioQueueRef};

    pub fn u32_transmute_ascii_str_le(string: &str) -> Result<u32, &'static str> {
        let bytes = (*string).as_bytes();
        if bytes.len() != 4 {
            return Err("Incorect number of bytes!");
        }
        let num_arr: [u8; 4] = bytes.try_into().unwrap();
        let num = u32::from_le_bytes(num_arr);
        Ok(num)
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
        pub num: u8
    }
}

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(dead_code)]
pub mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
