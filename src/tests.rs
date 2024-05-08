use crate::audio::Audio;

#[test]
pub fn test_get_num_devices() {
    let num_devices = Audio::num_devices().unwrap();
    println!("NUM DEVICES: {num_devices}");
    assert!(num_devices > 0);
}

#[test]
pub fn test_get_device_ids() {
    let device_ids = Audio::get_device_ids().unwrap();
    println!("DEVICE IDS: {:#?}", device_ids);
    assert!(device_ids.len() > 0);
}

#[test]
pub fn test_get_device_name() {
    let device_ids = Audio::get_device_ids().unwrap();
    let device_name = Audio::get_device_name(&device_ids[0]);
    println!("DEVICE NAME: {device_name}");
    assert!(device_name.len() > 0);
}

#[test]
pub fn test_is_output_device() {
    let device_ids = Audio::get_device_ids().unwrap();
    // Loop until an output device is found
    for device_id in &device_ids {
        if Audio::is_output_device(device_id).unwrap() {
            let device_name = Audio::get_device_name(device_id);
            println!("OUTPUT DEVICE FOUND: {device_name}");
            assert!(true);
            return;
        }
    }
    println!("NO OUTPUT DEVICE FOUND!");
    assert!(false);
}

#[test]
pub fn test_get_device_names() {
    let device_names = Audio::get_device_names().unwrap();
    println!("DEVICE NAMES: {:#?}", device_names);
    assert!(device_names.len() > 0);
}