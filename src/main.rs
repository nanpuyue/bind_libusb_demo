use std::ffi::CString;
use std::mem::{forget, MaybeUninit};
use std::os::raw::c_char;
use std::ptr::null_mut;

use self::libusb::*;

#[allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals
)]
mod libusb {
    include!(concat!(env!("OUT_DIR"), "/libusb.rs"));
}

fn main() {
    let mut ctx = null_mut();
    unsafe {
        if libusb_init(&mut ctx) == 0 {
            let mut list = null_mut();
            let ret = libusb_get_device_list(ctx, &mut list);
            if ret > 0 {
                let list = core::slice::from_raw_parts_mut(list, ret as usize);
                for device in list.iter() {
                    let mut handle = null_mut();
                    let ret = libusb_open(*device, &mut handle);
                    if ret == 0 {
                        let mut desc = MaybeUninit::uninit();
                        let ret = libusb_get_device_descriptor(*device, desc.as_mut_ptr());
                        if ret != 0 {
                            eprintln!("libusb_get_device_descriptor() err: {}!", ret);
                            libusb_close(handle);
                            break;
                        }
                        let desc = desc.assume_init();
                        let mut buf = Vec::with_capacity(1024);
                        let ret = libusb_get_string_descriptor_ascii(
                            handle,
                            desc.iProduct,
                            buf.as_mut_ptr(),
                            1024,
                        );
                        if ret > 0 {
                            let desc = CString::from_raw(buf.as_mut_ptr() as *mut c_char)
                                .into_string()
                                .unwrap();
                            forget(buf);
                            println!("{}", desc);
                        }
                        libusb_close(handle);
                    } else {
                        eprintln!("libusb_open() err: {}!", ret);
                    }
                }
                libusb_free_device_list(list.as_mut_ptr(), 1);
            }
            libusb_exit(ctx);
        }
    }
}
