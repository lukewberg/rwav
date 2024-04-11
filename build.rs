use std::{env, path::PathBuf, process::Command};

fn main() {
    #[cfg(target_os = "macos")]
    link_macos();

    #[cfg(target_os = "windows")]
    link_windows();
}

#[cfg(target_os = "macos")]
fn link_macos() {
        // Tell cargo to look for shared libraries in the specified directory
    // println!("cargo:rustc-link-search=/path/to/lib");

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=framework=AudioToolbox");
    println!("cargo:rustc-link-lib=framework=CoreFoundation");
    println!("cargo:rustc-link-lib=framework=CoreAudio");

    // Locate the AudioToolbox sdk
    let mut command = Command::new("xcrun");
    let child = command
        .args(["--sdk", "macosx", "--show-sdk-path"])
        .output()
        .expect("Failed to spawn command process!");

    if !child.status.success() {
        // Handle the case where xcrun fails
        eprintln!(
            "Error running xcrun: {}",
            String::from_utf8_lossy(&child.stderr)
        );
        std::process::exit(1);
    }

    let binding = String::from_utf8(child.stdout).unwrap();
    let sdk_path = binding.lines().next().unwrap();
    println!("SDK PATH {}", sdk_path);

    let bindings = bindgen::Builder::default()
        .header(format!(
            "{}/System/Library/Frameworks/AudioToolbox.framework/Headers/AudioToolbox.h",
            sdk_path
        ))
        // .header(format!(
        //     "{}/System/Library/Frameworks/CoreAudioTypes.framework/Headers/CoreAudioTypes.h",
        //     sdk_path
        // ))
        .objc_extern_crate(false)
        .block_extern_crate(false)
        .generate_block(true)
        .clang_args(["-isysroot", sdk_path, "-x", "objective-c", "-fblocks"])
        .allowlist_function("AudioQueueNewOutput")
        .allowlist_function("AudioQueueStart")
        .allowlist_function("AudioQueuePrime")
        .allowlist_function("AudioQueueFlush")
        .allowlist_function("AudioQueueStop")
        .allowlist_function("AudioQueuePause")
        .allowlist_function("AudioQueueReset")
        .allowlist_function("AudioQueueDispose")
        .allowlist_function("AudioQueueAllocateBuffer")
        .allowlist_function("AudioQueueEnqueueBuffer")
        .allowlist_function("AudioObjectGetPropertyData")
        .allowlist_function("AudioObjectGetPropertyDataSize")
        .allowlist_function("CFRunLoopGetCurrent")
        .allowlist_function("CFRunLoopRun")
        .allowlist_function("CFRunLoopStop")
        .allowlist_item("kCFRunLoopDefaultMode")
        .allowlist_item("kCFRunLoopCommonModes")
        .allowlist_item("AudioDeviceId")
        .allowlist_item("CFString")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings!");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

#[cfg(target_os = "windows")]
fn link_windows() {

}