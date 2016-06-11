extern crate winapi;
extern crate user32;
use std::ptr;
use winapi::winuser::MONITORINFO;
use winapi::minwindef::*;
use winapi::windef::*;
use std::mem;
// use std::fmt::Debug;

#[allow(non_snake_case, unused_variables)]
unsafe extern "system" fn callback(monitor: HMONITOR, hdc: HDC, work_rect: LPRECT, d: LPARAM) -> BOOL {
    let mut monitorInfo: Box<MONITORINFO> = Box::new(MONITORINFO { cbSize: mem::size_of::<MONITORINFO>() as u32, rcMonitor: RECT {left: 0, top: 0, right: 0, bottom: 0}, rcWork: RECT {left: 0, top: 0, right: 0, bottom: 0}, dwFlags: 0 });
    user32::GetMonitorInfoW(monitor, monitorInfo.as_mut());
    if monitorInfo.dwFlags == 1 {
        let mut actual_dimensions = Box::new(ScreenDimensions { width: monitorInfo.rcMonitor.right, height: monitorInfo.rcMonitor.bottom });
        ptr::copy(actual_dimensions.as_mut(), d as *mut ScreenDimensions, mem::size_of::<ScreenDimensions>());
        return 0;
    }
    1
}

fn main() {
    let mut screen_dimensions: Box<ScreenDimensions> = Box::new(ScreenDimensions { width: 0, height: 0 });
    let raw = Box::into_raw(screen_dimensions);
    unsafe { user32::EnumDisplayMonitors(ptr::null_mut(), ptr::null_mut(), Some(callback), raw as i64); }
    screen_dimensions = unsafe { Box::from_raw(raw) };
    println!("hi! Your primary screen is {}px by {}px", screen_dimensions.width, screen_dimensions.height);
}

struct ScreenDimensions {
    width: i32,
    height: i32
}