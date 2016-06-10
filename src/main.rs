extern crate winapi;
extern crate user32;
use std::ptr;
use winapi::winuser::MONITORENUMPROC;
use winapi::winuser::LPMONITORINFO;
use winapi::winuser::MONITORINFO;
use winapi::minwindef::*;
use winapi::windef::*;
use std::mem;
use std::fmt::Debug;

unsafe extern "system" fn callback(monitor: HMONITOR, hdc: HDC, work_rect: LPRECT, d: LPARAM) -> BOOL {
    // let monitorInfoPtr: LPMONITORINFO  = ptr::null_mut();
    let mut monitorInfo: Box<MONITORINFO> = Box::new(MONITORINFO { cbSize: 0, rcMonitor: RECT {left: 0, top: 0, right: 0, bottom: 0}, rcWork: RECT {left: 0, top: 0, right: 0, bottom: 0}, dwFlags: 0 });
    monitorInfo.cbSize = mem::size_of::<MONITORINFO>() as u32;
    user32::GetMonitorInfoW(monitor, monitorInfo.as_mut());
    println!("Got monitor info: {:?}", monitorInfo);
    if monitorInfo.dwFlags == 1 {
        return 0;
    }
    1
}

fn main() {
    unsafe { user32::EnumDisplayMonitors(ptr::null_mut(), ptr::null_mut(), Some(callback), 0); }
    println!("hi");
}
