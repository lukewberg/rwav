use std::{
    ffi::{c_void, CString},
    mem::size_of,
    ptr,
};

#[cfg(target_os = "windows")]
use windows::Media::AudioBuffer;

use crate::bindings::{flags::AudioDeviceId, AudioObjectGetPropertyData, CFStringRef, __CFString};
#[cfg(target_os = "macos")]
use crate::{
    bindings::{
        self, flags::kAudioObjectSystemObject, AudioObjectID, AudioObjectPropertyAddress, UInt32,
    },
    utils,
};

pub struct Audio(());

impl Audio {
    #[cfg(target_os = "macos")]
    fn num_devices_darwin() -> Result<u32, bindings::OSStatus> {
        let selector: u32 =
            utils::ascii_str_transmute_u32_be("dev#").expect("Unable to transmute bytes!");
        let scope: u32 =
            utils::ascii_str_transmute_u32_be("glob").expect("Unable to transmute bytes!");

        let property_address = AudioObjectPropertyAddress {
            mSelector: selector,
            mScope: scope,
            mElement: 0,
        };

        let mut data_size: u32 = 0;

        unsafe {
            let os_status = bindings::AudioObjectGetPropertyDataSize(
                kAudioObjectSystemObject,
                &property_address,
                0,
                ptr::null(),
                &mut data_size as *mut UInt32,
            );

            if os_status > 0_i32 {
                return Err(os_status);
            }

            data_size = data_size / size_of::<AudioObjectID>() as u32;
            Ok(data_size)
        }
    }

    pub fn device_id_darwin() {
        let mut data_size: u32 = 0;
        let mut device_id: UInt32 = 0;

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
        unsafe {
            let os_status = bindings::AudioObjectGetPropertyData(
                kAudioObjectSystemObject,
                &property_address,
                0,
                ptr::null(),
                &mut data_size as *mut UInt32,
                utils::get_cvoid_ptr(&mut device_id),
            );
        }
    }

    #[cfg(target_os = "windows")]
    fn num_devices_nt() {
        println!("Hello world, NT!")
    }

    pub fn num_devices() -> u32 {
        #[cfg(target_os = "windows")]
        return Self::num_devices_nt();

        #[cfg(target_os = "macos")]
        return Self::num_devices_darwin().unwrap();
    }

    #[cfg(target_os = "macos")]
    fn get_device_ids_darwin() -> Result<Vec<AudioDeviceId>, bindings::OSStatus> {
        let num_devices = Self::num_devices();
        let mut devices: Vec<AudioDeviceId> = vec![0u32; num_devices as usize];
        let selector: u32 =
            utils::ascii_str_transmute_u32_be("dev#").expect("Unable to transmute bytes!");
        let scope: u32 =
            utils::ascii_str_transmute_u32_be("glob").expect("Unable to transmute bytes!");

        let property_address = AudioObjectPropertyAddress {
            mElement: 0,
            mScope: scope,
            mSelector: selector,
        };

        let mut device_size = num_devices * size_of::<u32>() as u32;
        unsafe {
            let os_status = AudioObjectGetPropertyData(
                kAudioObjectSystemObject,
                &property_address,
                0,
                std::ptr::null(),
                &mut device_size as *mut UInt32,
                utils::get_cvoid_ptr(&mut *devices),
            );

            if os_status > 0 {
                return Err(os_status);
            }
        }
        Ok(devices)
    }

    pub fn get_device_ids() -> Result<Vec<u32>, i32> {
        #[cfg(target_os = "macos")]
        {
            return Self::get_device_ids_darwin();
        }
    }

    fn get_device_name_darwin(device_id: &AudioDeviceId) -> Result<String, String> {
        // I guess CFStringRef is now a typealias for CFString, which is opaque and apparently size 0.
        // We'll use the ref type because it's the same and has a size

        let mut property_size = size_of::<CFStringRef>() as UInt32;

        let selector: u32 =
            utils::ascii_str_transmute_u32_be("dev#").expect("Unable to transmute bytes!");
        let scope: u32 =
            utils::ascii_str_transmute_u32_be("glob").expect("Unable to transmute bytes!");
        let element =
            utils::ascii_str_transmute_u32_be("lnam").expect("Unable to transmute bytes!");

        let property_address = AudioObjectPropertyAddress {
            mElement: element, // kAudioDevicePropertyDeviceNameCFString
            mScope: scope,
            mSelector: selector,
        };

        let mut result = ptr::dangling::<CString>();

        unsafe {
            let os_status = AudioObjectGetPropertyData(
                *device_id,
                &property_address,
                0,
                std::ptr::null(),
                &mut property_size as *mut UInt32,
                utils::get_cvoid_ptr(&mut result),
            );

            if os_status == 0 {
                return Err(utils::u32_transmute_ascii_str_le(os_status as u32).unwrap());
            }
        }
        // Ok(String::from(result))
        Ok(String::new())
    }

    pub fn get_device_name(id: &u32) -> String {
        return Self::get_device_name_darwin(id).unwrap();
    }
}

pub struct SourceDescription {
    pub bits_per_sample: u16,
    pub num_channels: u16,
    pub sample_rate: u32,
}
