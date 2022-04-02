#![allow(unused_parens)]

use gms_binder::*;
use config_box::file_watcher::FileWatcherConfigBox;
use crossbeam_epoch::{self as epoch};

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

gms_bind_start!("ld50_lib", "ld50_lib.dll", "lib");

static mut CONFIG_BOX : Option<FileWatcherConfigBox> = None;

#[no_mangle]
#[gms_bind]
pub extern "C" fn dummy() -> f64 {
    println!("Hello");
    0.0
}

#[no_mangle]
#[gms_bind]
pub extern "C" fn setup_config_box(path_raw : *const c_char) -> f64 {
    unsafe {
        let path = CStr::from_ptr(path_raw).to_str().unwrap();
        CONFIG_BOX = Some(FileWatcherConfigBox::new(path).expect("Could not create configbox"));
    }

    0.0
}

#[no_mangle]
#[gms_bind]
pub extern "C" fn config_box_tick() -> f64 {
    unsafe {
        let config_box = CONFIG_BOX.as_ref().unwrap();
        config_box.refresh();
    }

    0.0
}

#[no_mangle]
#[gms_bind]
pub extern "C" fn config_f64(path_raw : *const c_char) -> f64 {
    unsafe {
        let config_box = CONFIG_BOX.as_ref().unwrap();
        let path = CStr::from_ptr(path_raw).to_str().unwrap();
        let guard = &epoch::pin();
        let root = config_box.root(guard);
        root.get_f32(path) as f64
    }
}

#[no_mangle]
#[gms_bind]
// Warning leaks
pub extern "C" fn config_str(path_raw : *const c_char) -> *mut c_char  {
    unsafe {
        let config_box = CONFIG_BOX.as_ref().unwrap();
        let path = CStr::from_ptr(path_raw).to_str().unwrap();
        let guard = &epoch::pin();
        let root = config_box.root(guard);

        let value_str = root.get_str(path);

        CString::new(value_str).unwrap().into_raw()
    }
}

gms_bind_end!();