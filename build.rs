use std::{env, io::Read, path::PathBuf, process::Command};

fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    // println!("cargo:rustc-link-search=/path/to/lib");

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=framework=AudioToolbox");
    println!("cargo:rustc-link-lib=framework=CoreFoundation");

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
        .allowlist_function("AudioQueueAllocateBuffer")
        .allowlist_function("CFRunLoopGetCurrent")
        .allowlist_item("kCFRunLoopDefaultMode")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings!");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
