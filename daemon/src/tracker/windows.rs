use anyhow::Result;
use windows::{
    core::PWSTR,
    Win32::{
        Foundation::{HWND, LPARAM, WPARAM},
        System::ProcessStatus::GetModuleBaseNameW,
        System::Threading::{GetCurrentProcess, OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ},
        UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowTextW, GetWindowThreadProcessId},
    },
};

use crate::types::ActiveWindow;

pub async fn get_active_window() -> Result<ActiveWindow> {
    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd.0 == 0 {
            return Ok(ActiveWindow {
                app_name: "Desktop".to_string(),
                window_title: "Desktop".to_string(),
                process_id: 0,
            });
        }

        // Get window title
        let mut title_buffer = [0u16; 512];
        let title_len = GetWindowTextW(hwnd, &mut title_buffer);
        let window_title = if title_len > 0 {
            String::from_utf16_lossy(&title_buffer[..title_len as usize])
        } else {
            "Unknown".to_string()
        };

        // Get process ID
        let mut process_id = 0u32;
        GetWindowThreadProcessId(hwnd, Some(&mut process_id));

        // Get process name
        let app_name = if process_id != 0 {
            get_process_name(process_id).unwrap_or_else(|_| "Unknown".to_string())
        } else {
            "Unknown".to_string()
        };

        Ok(ActiveWindow {
            app_name,
            window_title,
            process_id,
        })
    }
}

unsafe fn get_process_name(process_id: u32) -> Result<String> {
    let process_handle = OpenProcess(
        PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
        false,
        process_id,
    )?;

    let mut module_name = [0u16; 260];
    let result = GetModuleBaseNameW(
        process_handle,
        None,
        &mut module_name,
    );

    if result > 0 {
        Ok(String::from_utf16_lossy(&module_name[..result as usize]))
    } else {
        Ok("Unknown".to_string())
    }
}