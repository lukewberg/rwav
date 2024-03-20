use std::{ffi::c_void, mem::size_of, ptr};

use crate::{
    bindings::{self, kAudioObjectSystemObject, AudioObjectID, AudioObjectPropertyAddress, UInt32},
    utils,
};

pub struct Audio {}

impl Audio {
    pub fn new() -> Self {
        Audio {}
    }

    pub fn get_devices() {
        unsafe {
            let selector: [u8; 4] = "dev#"
                .as_bytes()
                .try_into()
                .expect("Unable to transmute bytes!");
            let scope: [u8; 4] = "glob"
                .as_bytes()
                .try_into()
                .expect("Unable to transmute bytes!");

            let property_address = AudioObjectPropertyAddress {
                mSelector: u32::from_be_bytes(selector),
                mScope: u32::from_be_bytes(scope),
                mElement: 0,
            };

            let mut data_size: u32 = 0;
            let mut device_id: UInt32 = 0;
            let this_status = bindings::AudioObjectGetPropertyDataSize(
                kAudioObjectSystemObject,
                &property_address,
                0,
                ptr::null(),
                &mut data_size as *mut u32,
            );

            let os_status = bindings::AudioObjectGetPropertyData(
                kAudioObjectSystemObject,
                &property_address,
                0,
                ptr::null(),
                &mut data_size as *mut u32,
                utils::get_cvoid_ptr(&mut device_id),
            );
            data_size = data_size / size_of::<AudioObjectID>() as u32;
            println!("{os_status}");
        }
    }
}
