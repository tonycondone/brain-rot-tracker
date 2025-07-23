use anyhow::Result;
use std::ffi::CStr;
use std::ptr;
use x11::xlib::{
    Display, Window, XCloseDisplay, XFetchName, XFree, XGetInputFocus, XGetWindowProperty,
    XOpenDisplay, XQueryTree, _NET_ACTIVE_WINDOW, _NET_WM_NAME, _NET_WM_PID,
};

use crate::types::ActiveWindow;

pub async fn get_active_window() -> Result<ActiveWindow> {
    unsafe {
        let display = XOpenDisplay(ptr::null());
        if display.is_null() {
            return Ok(ActiveWindow {
                app_name: "Desktop".to_string(),
                window_title: "Desktop".to_string(),
                process_id: 0,
            });
        }

        let result = get_active_window_info(display);
        XCloseDisplay(display);
        
        result
    }
}

unsafe fn get_active_window_info(display: *mut Display) -> Result<ActiveWindow> {
    let root = x11::xlib::XDefaultRootWindow(display);
    
    // Try to get the active window using _NET_ACTIVE_WINDOW
    let active_window = get_active_window_from_property(display, root);
    
    let window = if active_window != 0 {
        active_window
    } else {
        // Fallback to input focus
        let mut focus_window = 0;
        let mut revert_to = 0;
        XGetInputFocus(display, &mut focus_window, &mut revert_to);
        focus_window
    };

    if window == 0 {
        return Ok(ActiveWindow {
            app_name: "Desktop".to_string(),
            window_title: "Desktop".to_string(),
            process_id: 0,
        });
    }

    let window_title = get_window_title(display, window);
    let process_id = get_window_pid(display, window);
    let app_name = get_process_name(process_id).unwrap_or_else(|| "Unknown".to_string());

    Ok(ActiveWindow {
        app_name,
        window_title,
        process_id,
    })
}

unsafe fn get_active_window_from_property(display: *mut Display, root: Window) -> Window {
    let net_active_window = x11::xlib::XInternAtom(display, b"_NET_ACTIVE_WINDOW\0".as_ptr() as *const i8, 0);
    
    let mut actual_type = 0;
    let mut actual_format = 0;
    let mut nitems = 0;
    let mut bytes_after = 0;
    let mut prop: *mut u8 = ptr::null_mut();

    let result = XGetWindowProperty(
        display,
        root,
        net_active_window,
        0,
        1,
        0,
        x11::xlib::XA_WINDOW,
        &mut actual_type,
        &mut actual_format,
        &mut nitems,
        &mut bytes_after,
        &mut prop,
    );

    if result == 0 && !prop.is_null() && nitems > 0 {
        let window = *(prop as *const Window);
        XFree(prop as *mut _);
        window
    } else {
        0
    }
}

unsafe fn get_window_title(display: *mut Display, window: Window) -> String {
    let mut name: *mut i8 = ptr::null_mut();
    
    if XFetchName(display, window, &mut name) != 0 && !name.is_null() {
        let title = CStr::from_ptr(name).to_string_lossy().to_string();
        XFree(name as *mut _);
        title
    } else {
        "Unknown".to_string()
    }
}

unsafe fn get_window_pid(display: *mut Display, window: Window) -> u32 {
    let net_wm_pid = x11::xlib::XInternAtom(display, b"_NET_WM_PID\0".as_ptr() as *const i8, 0);
    
    let mut actual_type = 0;
    let mut actual_format = 0;
    let mut nitems = 0;
    let mut bytes_after = 0;
    let mut prop: *mut u8 = ptr::null_mut();

    let result = XGetWindowProperty(
        display,
        window,
        net_wm_pid,
        0,
        1,
        0,
        x11::xlib::XA_CARDINAL,
        &mut actual_type,
        &mut actual_format,
        &mut nitems,
        &mut bytes_after,
        &mut prop,
    );

    if result == 0 && !prop.is_null() && nitems > 0 {
        let pid = *(prop as *const u32);
        XFree(prop as *mut _);
        pid
    } else {
        0
    }
}

fn get_process_name(pid: u32) -> Option<String> {
    if pid == 0 {
        return None;
    }

    let comm_path = format!("/proc/{}/comm", pid);
    std::fs::read_to_string(comm_path)
        .ok()
        .map(|s| s.trim().to_string())
}