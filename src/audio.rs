use std::{mem::size_of, ptr};

use thiserror::Error;
#[cfg(target_os = "windows")]
use windows::Media::AudioBuffer;

#[cfg(target_os = "macos")]
use crate::{
    bindings::{
        self, flags::kAudioObjectSystemObject, flags::AudioDeviceId, AudioObjectGetPropertyData,
        AudioObjectID, AudioObjectPropertyAddress, AudioObjectSetPropertyData, CFRange,
        CFStringGetCharacters, CFStringGetLength, CFStringRef, UInt32,
    },
    utils,
    utils::{create_cfstring_from_rust, get_cvoid_ptr, release_cfstring},
};

pub struct Audio(());

impl Audio {
    #[inline(always)]
    #[cfg(target_os = "macos")]
    fn set_output_device_darwin(device_id: &AudioDeviceId) -> Result<bool, AudioOperationError> {
        let mut device_id_copy = *device_id;
        let property_size = std::mem::size_of::<u32>() as u32;
        let property_address = AudioObjectPropertyAddress {
            mSelector: 1682929012,
            mScope: 1735159650,
            mElement: 0,
        };

        let os_status = unsafe {
            AudioObjectSetPropertyData(
                1,
                &property_address,
                0,
                std::ptr::null(),
                property_size,
                utils::get_cvoid_ptr(&mut device_id_copy),
            )
        };

        if os_status > 0 {
            return Err(AudioOperationError::SystemCommError);
        }
        Ok(true)
    }

    pub fn set_output_device(device_id: &AudioDeviceId) -> Result<bool, AudioOperationError> {
        #[cfg(target_os = "macos")]
        {
            Self::set_output_device_darwin(device_id)
        }
    }

    #[inline(always)]
    #[cfg(target_os = "macos")]
    fn get_devices_darwin() -> Result<Vec<(String, AudioDeviceId)>, AudioOperationError> {
        let device_ids = Self::get_device_ids()?;
        let mut result: Vec<(String, AudioDeviceId)> = Vec::with_capacity(device_ids.len());
        for device in device_ids {
            let name = Self::get_device_name(&device);
            result.push((name, device));
        }
        Ok(result)
    }

    pub fn get_devices() -> Result<Vec<(String, AudioDeviceId)>, AudioOperationError> {
        #[cfg(target_os = "macos")]
        {
            Self::get_devices_darwin()
        }
    }

    #[inline(always)]
    #[cfg(target_os = "macos")]
    fn set_device_volume_darwin(
        device_id: &AudioDeviceId,
        left_channel: f32,
        right_channel: f32,
    ) -> Result<bool, AudioOperationError> {
        let mut channels = vec![0u32; 2];
        let mut property_size = (std::mem::size_of::<u32>() * 2) as u32;
        let mut left_level = left_channel;
        let mut right_level = right_channel;

        let mut property_address = AudioObjectPropertyAddress {
            mSelector: 1684236338,
            mScope: 1869968496,
            mElement: 0,
        };

        let mut os_status = unsafe {
            AudioObjectGetPropertyData(
                *device_id,
                &property_address,
                0,
                std::ptr::null(),
                &mut property_size,
                utils::get_cvoid_ptr(&mut channels),
            )
        };

        if os_status > 0 {
            return Err(AudioOperationError::SystemCommError);
        }

        property_address.mSelector = 1987013741;
        property_size = std::mem::size_of::<f32>() as u32;
        property_address.mElement = channels[0];

        os_status = unsafe {
            AudioObjectSetPropertyData(
                *device_id,
                &property_address,
                0,
                std::ptr::null(),
                property_size,
                utils::get_cvoid_ptr(&mut left_level),
            )
        };

        if os_status > 0 {
            return Err(AudioOperationError::SystemCommError);
        }

        property_address.mElement = channels[0];

        os_status = unsafe {
            AudioObjectSetPropertyData(
                *device_id,
                &property_address,
                0,
                std::ptr::null(),
                property_size,
                utils::get_cvoid_ptr(&mut right_level),
            )
        };

        if os_status > 0 {
            return Err(AudioOperationError::SystemCommError);
        }

        Ok(true)
    }

    pub fn set_device_volume(
        device_id: &AudioDeviceId,
        left_channel: f32,
        right_channel: f32,
    ) -> Result<bool, AudioOperationError> {
        #[cfg(target_os = "macos")]
        {
            Self::set_device_volume_darwin(device_id, left_channel, right_channel)
        }
    }

    #[inline(always)]
    #[cfg(target_os = "macos")]
    fn get_device_names_darwin() -> Result<Vec<String>, AudioOperationError> {
        let device_ids = Self::get_device_ids_darwin()?;
        let mut device_names: Vec<String> = Vec::with_capacity(device_ids.len());
        for device_id in device_ids {
            let device_name = Self::get_device_name_darwin(&device_id)?;
            device_names.push(device_name);
        }
        Ok(device_names)
    }

    pub fn get_device_names() -> Result<Vec<String>, AudioOperationError> {
        #[cfg(target_os = "macos")]
        {
            return Self::get_device_names_darwin();
        }
    }

    #[inline(always)]
    #[cfg(target_os = "macos")]
    fn is_output_device_darwin(device_id: &AudioDeviceId) -> Result<bool, AudioOperationError> {
        let mut property_size = 256u32;
        let property_address = AudioObjectPropertyAddress {
            mElement: 0,
            mScope: 1869968496,
            mSelector: 1937009955,
        };

        let os_status = unsafe {
            bindings::AudioObjectGetPropertyDataSize(
                *device_id,
                &property_address,
                0,
                ptr::null(),
                &mut property_size as *mut UInt32,
            )
        };
        if os_status > 0 {
            return Err(AudioOperationError::UnableToGetDeviceDescription);
        }
        Ok(property_size > 0)
    }

    pub fn is_output_device(device_id: &AudioDeviceId) -> Result<bool, AudioOperationError> {
        #[cfg(target_os = "macos")]
        {
            return Self::is_output_device_darwin(device_id);
        }
    }

    #[inline(always)]
    #[cfg(target_os = "macos")]
    fn num_devices_darwin() -> Result<u32, AudioOperationError> {
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
                return Err(AudioOperationError::UnableToGetNumDevices);
            }

            data_size = data_size / size_of::<AudioObjectID>() as u32;
            Ok(data_size)
        }
    }

    #[cfg(target_os = "windows")]
    fn num_devices_nt() {
        println!("Hello world, NT!")
    }

    pub fn num_devices() -> Result<u32, AudioOperationError> {
        #[cfg(target_os = "windows")]
        return Ok(Self::num_devices_nt());

        #[cfg(target_os = "macos")]
        return Ok(Self::num_devices_darwin()?);
    }

    #[inline(always)]
    #[cfg(target_os = "macos")]
    fn get_device_ids_darwin() -> Result<Vec<AudioDeviceId>, AudioOperationError> {
        let num_devices = Self::num_devices().expect("Oops!");
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
                return Err(AudioOperationError::UnableToGetDeviceId);
            }
        }
        Ok(devices)
    }

    pub fn get_device_ids() -> Result<Vec<AudioDeviceId>, AudioOperationError> {
        #[cfg(target_os = "macos")]
        {
            return Self::get_device_ids_darwin();
        }
    }

    #[inline(always)]
    #[cfg(target_os = "macos")]
    fn get_device_name_darwin(device_id: &AudioDeviceId) -> Result<String, AudioOperationError> {
        // I guess CFStringRef is now a typealias for CFString, which is opaque and apparently size 0.
        // We'll use the ref type because it's the same and has a size

        let mut property_size = size_of::<CFStringRef>() as UInt32;

        let element: u32 = 0u32;
        let scope: u32 =
            utils::ascii_str_transmute_u32_be("glob").expect("Unable to transmute bytes!");
        let selector =
            utils::ascii_str_transmute_u32_be("lnam").expect("Unable to transmute bytes!");

        let property_address = AudioObjectPropertyAddress {
            mElement: element, // kAudioDevicePropertyDeviceNameCFString
            mScope: scope,
            mSelector: selector,
        };

        let mut result = create_cfstring_from_rust("");
        let result_ptr = utils::get_cvoid_ptr(&mut result);

        unsafe {
            let os_status = AudioObjectGetPropertyData(
                *device_id,
                &property_address,
                0,
                std::ptr::null(),
                &mut property_size as *mut UInt32,
                result_ptr,
            );

            if os_status != 0 {
                return Err(AudioOperationError::DeviceNotFound);
            }
        }

        let name = unsafe {
            let length = CFStringGetLength(result);
            let mut buffer = vec![0u16; length as usize];
            CFStringGetCharacters(
                result,
                CFRange {
                    location: 0,
                    length,
                },
                buffer.as_mut_ptr(),
            );
            release_cfstring(result);
            match String::from_utf16(&buffer) {
                Ok(name) => Ok(name),
                Err(_) => Err(AudioOperationError::UnableToGetDeviceName),
            }
        };
        Ok(name?)
    }

    pub fn get_device_name(id: &AudioDeviceId) -> String {
        return Self::get_device_name_darwin(id).unwrap();
    }
}

pub struct SourceDescription {
    pub bits_per_sample: u16,
    pub num_channels: u16,
    pub sample_rate: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SessionState {
    Initialized,
    Started,
    Paused,
    Stopped,
}

pub struct AudioSession {
    pub id: u32,
    pub source_description: SourceDescription,
    pub state: SessionState,
    pub device: Option<AudioDeviceId>,
}

impl AudioSession {

}

pub struct AudioManager {
    sessions: Vec<AudioSession>,
}

impl AudioManager {

    pub fn new() -> Self {
        Self {
            sessions: Vec::new()
        }
    }
}

#[derive(Error, Debug)]
pub enum AudioOperationError {
    #[error("Device not found!")]
    DeviceNotFound,
    #[error("Device not available!")]
    DeviceNotAvailable,
    #[error("Device not open!")]
    DeviceNotOpen,
    #[error("Device not closed!")]
    DeviceNotStarted,
    #[error("Device not stopped!")]
    DeviceNotStopped,
    #[error("Device not paused!")]
    DeviceNotPaused,
    #[error("Device not unpaused!")]
    DeviceNotUnpaused,
    #[error("Device not disposed!")]
    DeviceNotDisposed,
    #[error("Device not initialized!")]
    DeviceNotInitialized,
    #[error("Device not reset!")]
    DeviceNotReset,
    #[error("Device not suspended!")]
    DeviceNotSuspended,
    #[error("Device not resumed!")]
    DeviceNotResumed,
    #[error("Device not interrupted!")]
    DeviceNotInterrupted,
    #[error("Device not uninterrupted!")]
    DeviceNotUninterrupted,
    #[error("Device not started at time!")]
    DeviceNotStartedAtTime,
    #[error("Device not stopped at time!")]
    DeviceNotStoppedAtTime,
    #[error("Device not scheduled!")]
    DeviceNotScheduled,
    #[error("Device not unscheduled!")]
    DeviceNotUnscheduled,
    #[error("Device not flushed!")]
    DeviceNotFlushed,
    #[error("Device not enabled!")]
    DeviceNotEnabled,
    #[error("Device not disabled!")]
    DeviceNotDisabled,
    #[error("Unable to get device name from system!")]
    UnableToGetDeviceName,
    #[error("Unable to get device id from system!")]
    UnableToGetDeviceId,
    #[error("Unable to get number of devices from system!")]
    UnableToGetNumDevices,
    #[error("Unable to get device description from system!")]
    UnableToGetDeviceDescription,
    #[error("Unable to get device source description from system!")]
    UnableToGetDeviceSourceDescription,
    #[error("General error communicating with the system!")]
    SystemCommError,
}
