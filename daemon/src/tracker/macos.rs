use anyhow::Result;
use core_foundation::{
    array::{CFArray, CFArrayRef},
    base::{CFType, TCFType},
    dictionary::{CFDictionary, CFDictionaryRef},
    number::{CFNumber, CFNumberRef},
    string::{CFString, CFStringRef},
};
use core_graphics::window::{kCGWindowListOptionOnScreenOnly, CGWindowListCopyWindowInfo};
use std::ffi::c_void;

use crate::types::ActiveWindow;

pub async fn get_active_window() -> Result<ActiveWindow> {
    unsafe {
        let window_list = CGWindowListCopyWindowInfo(kCGWindowListOptionOnScreenOnly, 0);
        
        if window_list.is_null() {
            return Ok(ActiveWindow {
                app_name: "Desktop".to_string(),
                window_title: "Desktop".to_string(),
                process_id: 0,
            });
        }

        let array: CFArray<CFDictionary> = CFArray::wrap_under_create_rule(window_list as CFArrayRef);
        
        for i in 0..array.len() {
            if let Some(window_info) = array.get(i) {
                // Check if this is the frontmost window
                if let Some(layer) = get_window_layer(&window_info) {
                    if layer == 0 {
                        let app_name = get_window_owner_name(&window_info)
                            .unwrap_or_else(|| "Unknown".to_string());
                        let window_title = get_window_name(&window_info)
                            .unwrap_or_else(|| "Unknown".to_string());
                        let process_id = get_window_owner_pid(&window_info)
                            .unwrap_or(0);

                        return Ok(ActiveWindow {
                            app_name,
                            window_title,
                            process_id,
                        });
                    }
                }
            }
        }

        Ok(ActiveWindow {
            app_name: "Desktop".to_string(),
            window_title: "Desktop".to_string(),
            process_id: 0,
        })
    }
}

unsafe fn get_window_layer(window_info: &CFDictionary) -> Option<i32> {
    let key = CFString::from_static_string("kCGWindowLayer");
    window_info.find(&key)
        .and_then(|value_ref| {
            let number: CFNumber = CFNumber::wrap_under_get_rule(*value_ref as CFNumberRef);
            number.to_i32()
        })
}

unsafe fn get_window_owner_name(window_info: &CFDictionary) -> Option<String> {
    let key = CFString::from_static_string("kCGWindowOwnerName");
    window_info.find(&key)
        .map(|value_ref| {
            let string: CFString = CFString::wrap_under_get_rule(*value_ref as CFStringRef);
            string.to_string()
        })
}

unsafe fn get_window_name(window_info: &CFDictionary) -> Option<String> {
    let key = CFString::from_static_string("kCGWindowName");
    window_info.find(&key)
        .map(|value_ref| {
            let string: CFString = CFString::wrap_under_get_rule(*value_ref as CFStringRef);
            string.to_string()
        })
}

unsafe fn get_window_owner_pid(window_info: &CFDictionary) -> Option<u32> {
    let key = CFString::from_static_string("kCGWindowOwnerPID");
    window_info.find(&key)
        .and_then(|value_ref| {
            let number: CFNumber = CFNumber::wrap_under_get_rule(*value_ref as CFNumberRef);
            number.to_i32().map(|pid| pid as u32)
        })
}