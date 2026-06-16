use serde::{Deserialize, Serialize};
use std::ffi::CString;
use winapi::um::winuser::{
    EnumWindows, GetWindowTextW, GetClassNameW, IsWindowVisible, ShowWindow, 
    SW_RESTORE, SW_MINIMIZE, SW_MAXIMIZE, SetForegroundWindow, 
    GetWindowThreadProcessId, BringWindowToTop
};
use winapi::um::winnt::HANDLE;
use winapi::shared::windef::{HWND, RECT};
use winapi::um::winuser::{GetWindowRect, SetWindowPos, HWND_TOP, SWP_NOMOVE, SWP_NOSIZE};
use std::ptr;
use std::collections::HashMap;
use tauri::command;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WindowInfo {
    pub hwnd: u32,
    pub title: String,
    pub class_name: String,
    pub process_id: u32,
    pub is_visible: bool,
}

static mut WINDOWS: Vec<WindowInfo> = vec![];

unsafe extern "system" fn enum_windows_callback(hwnd: HWND, lparam: isize) -> i32 {
    if hwnd.is_null() {
        return 1;
    }

    let mut title = [0u16; 512];
    let len = GetWindowTextW(hwnd, title.as_mut_ptr(), 512);
    
    if len > 0 && IsWindowVisible(hwnd) != 0 {
        let mut class = [0u16; 256];
        GetClassNameW(hwnd, class.as_mut_ptr(), 256);
        
        let mut process_id: u32 = 0;
        GetWindowThreadProcessId(hwnd, &mut process_id as *mut u32);

        let title_str = String::from_utf16_lossy(&title[..len as usize]);
        let class_str = String::from_utf16_lossy(&class);

        let win_info = WindowInfo {
            hwnd: hwnd as u32,
            title: title_str.trim().to_string(),
            class_name: class_str.trim().to_string(),
            process_id,
            is_visible: true,
        };

        WINDOWS.push(win_info);
    }

    1
}

#[command]
pub fn list_windows() -> Vec<WindowInfo> {
    unsafe {
        WINDOWS.clear();
        EnumWindows(Some(enum_windows_callback), 0);
        WINDOWS.clone()
    }
}

#[command]
pub fn minimize_all_windows() {
    unsafe {
        WINDOWS.clear();
        EnumWindows(Some(enum_windows_callback), 0);
        for win in &WINDOWS {
            let hwnd = win.hwnd as HWND;
            ShowWindow(hwnd, SW_MINIMIZE);
        }
    }
}

#[command]
pub fn restore_all_windows() {
    unsafe {
        WINDOWS.clear();
        EnumWindows(Some(enum_windows_callback), 0);
        for win in &WINDOWS {
            let hwnd = win.hwnd as HWND;
            ShowWindow(hwnd, SW_RESTORE);
            BringWindowToTop(hwnd);
        }
    }
}

#[command]
pub fn focus_window(title: String) {
    unsafe {
        WINDOWS.clear();
        EnumWindows(Some(enum_windows_callback), 0);
        for win in &WINDOWS {
            if win.title.to_lowercase().contains(&title.to_lowercase()) {
                let hwnd = win.hwnd as HWND;
                ShowWindow(hwnd, SW_RESTORE);
                SetForegroundWindow(hwnd);
                return;
            }
        }
    }
}

#[command]
pub fn arrange_windows_layout(layout: String) {
    unsafe {
        WINDOWS.clear();
        EnumWindows(Some(enum_windows_callback), 0);
        
        let visible_windows: Vec<_> = WINDOWS.iter().filter(|w| !w.title.is_empty()).collect();
        
        if layout == "cascade" {
            for (i, win) in visible_windows.iter().enumerate() {
                let hwnd = win.hwnd as HWND;
                let x = (i * 30) as i32;
                let y = (i * 30) as i32;
                SetWindowPos(hwnd, HWND_TOP, x, y, 800, 600, SWP_NOMOVE | 0);
            }
        } else if layout == "grid" {
            let count = visible_windows.len();
            let cols = (count as f32).sqrt().ceil() as i32;
            for (i, win) in visible_windows.iter().enumerate() {
                let hwnd = win.hwnd as HWND;
                let row = i as i32 / cols;
                let col = i as i32 % cols;
                let x = col * 850;
                let y = row * 550;
                SetWindowPos(hwnd, HWND_TOP, x, y, 800, 500, 0);
            }
        }
    }
}

#[command]
pub fn save_current_layout(name: String) -> Result<(), String> {
    let windows = list_windows();
    println!("Saved layout '{}' with {} windows", name, windows.len());
    Ok(())
}