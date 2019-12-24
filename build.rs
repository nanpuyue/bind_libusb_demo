use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=usb-1.0");

    let bindings = bindgen::Builder::default()
        .header("/usr/include/libusb-1.0/libusb.h")
        .prepend_enum_name(false)
        .rustfmt_bindings(true)
        .whitelist_function("libusb_.+")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("unable to generate bindings!");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_dir.join("libusb.rs"))
        .expect("couldn't write bindings!");
}
