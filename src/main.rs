#[cfg(windows)] extern crate winapi;
use std::{thread, time};
use std::process::exit;

use livesplit_hotkey::{Hook, KeyCode};
use winapi::shared::windef::HWND;
use winapi::shared::minwindef::{LPARAM, BOOL, TRUE};
use winapi::um::winuser::{GetWindow, SetWindowPos, GetWindowTextA, EnumWindows,
                          WNDENUMPROC,  SWP_NOSIZE};
// use winapi::shared::minwindef::{FALSE};
// use winapi::um::winuser::{RegisterHotKey}
// use winapi::shared::minwindef::{DWORD, LRESULT, UINT, WPARAM};
// use parking_lot::Mutex;
// use std::collections::hash_map::{ HashMap, Entry };
// use std::sync::Arc;

// struct MyHook {
//     thread_id: DWORD,
//     hotkeys: Arc<Mutex<HashMap<KeyCode, Box<FnMut() + Send + 'static>>>>,
// }

#[cfg(windows)]
unsafe extern "system" fn select_calcs(hwnd: HWND, _lparam: LPARAM) -> BOOL {
    let windowtitle = get_window_title(hwnd);
    if windowtitle == "Calculator" {
        let nexthwnd = GetWindow(hwnd, 2);
        let _resizeresult = SetWindowPos(hwnd, nexthwnd, 0,0,0, 0, SWP_NOSIZE);
    }
    TRUE
}

fn get_window_title(hwnd: HWND) -> String {
    let size = 256;
    let mut buf = Vec::with_capacity(size as usize);
    unsafe {
        let winstr = GetWindowTextA(hwnd, buf.as_mut_ptr(), size);
        buf.set_len(winstr as usize);
    }
    let windowtitle = unsafe {
        let mut bufchars = std::mem::transmute::<Vec<i8>, Vec<u8>>(buf);
        bufchars.truncate(bufchars.len());
        let windowtitle = {String::from_utf8_lossy(&bufchars.clone()).into_owned()};
        windowtitle
    };
    windowtitle
}

fn die() -> () {
    println!("Dieing!");
    exit(0)
}

fn organize_calc() -> () {
    println!("Organizing calculators");
    let enumfun: WNDENUMPROC = Some(select_calcs);
    unsafe { EnumWindows(enumfun, isize::max_value()); }
}

fn main() -> Result<(), livesplit_hotkey::windows::Error> {
    /// Register global hotkeys, then sleep and handle events as callbacks
    println!("Registering hooks");
    let hook = Hook::new()?;
    hook.register(KeyCode::NumPad1, organize_calc)?;
    hook.register(KeyCode::NumPad2, die)?;

    let sleep_time = time::Duration::from_millis(10000);

    loop { thread::sleep(sleep_time);
        //println!("zzzzz");
        //let myhook: MyHook = unsafe {std::mem::transmute_copy::<Hook,MyHook>(&hook)};
        //println!("{}", myhook.thread_id);
    };
    Ok(())
}

