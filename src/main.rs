// Windows API Working with strings: https://docs.microsoft.com/en-us/windows/win32/learnwin32/working-with-strings

// TODO: The shizzle do this do? (extern crate)
extern crate winapi;

use std::{mem, ptr::null_mut};

use libloaderapi::GetModuleHandleW;
use minwindef::{HMODULE, LPARAM, LRESULT, UINT, WPARAM};
use winapi::{um::winuser};
use winuser::{CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, CreateWindowExW, DefWindowProcW, IDC_ARROW, LoadCursorW, RegisterClassExW, SW_SHOW, ShowWindow, WNDCLASSEXW, WS_MINIMIZEBOX, WS_OVERLAPPED, WS_OVERLAPPEDWINDOW};
use winapi::shared::minwindef;
use winapi::shared::windef::HWND;
use winapi::um::libloaderapi;

use std::iter::once;
use std::ffi::OsStr;

// rust-analyzer has an issue with unresolved import errors for platform specific modules such as std::os
// GitHub Issue: https://github.com/rust-analyzer/rust-analyzer/issues/6038
// OsStrExt contains Windows specific extensions to OsStr: encode_wide, which re-encodes an OsStr as UTF-16.
use std::os::windows::ffi::OsStrExt;

fn main() {
    println!("Hello, World!");

    // TODO: Read up on unsafe block
    unsafe {
        // TODO: What does this magic do?
        let class_name : Vec<u16> = OsStr::new("mainwindow").encode_wide().chain( once(0) ).collect();
        let hInstance : HMODULE = GetModuleHandleW(null_mut());

        let window_class = WNDCLASSEXW {
            // TODO: Read up on generics in Rust (including it's weird syntax)
            cbSize: mem::size_of::<WNDCLASSEXW>() as UINT,
            style: CS_HREDRAW | CS_VREDRAW,
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: hInstance,
            hIcon: null_mut(),
            hIconSm: null_mut(),
            hCursor: LoadCursorW(hInstance, IDC_ARROW),
            hbrBackground: null_mut(),
            lpszMenuName: null_mut(),
            lpszClassName: class_name.as_ptr(),
            lpfnWndProc: Some(window_proc)
        };

        // RegisterClassExW returns 0 if it fails
        // https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw
        if RegisterClassExW(&window_class) == 0 {
            println!("Failed to register window class!");
            return
        }

        let window_title : Vec<u16> = OsStr::new("Rusty Beagle!").encode_wide().chain( once(0) ).collect();

        // https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexa
        let main_window = CreateWindowExW(
            0, 
            class_name.as_ptr(), 
            window_title.as_ptr(), 
            WS_OVERLAPPEDWINDOW, 
            CW_USEDEFAULT, CW_USEDEFAULT, 800, 600, 
            null_mut(), 
            null_mut(), 
            hInstance, 
            null_mut());

        if main_window.is_null() {
            println!("Failed to create window! :(");
            return
        }

        ShowWindow(main_window, SW_SHOW);

        let should_quit = false;

        while (!should_quit) {
            
        }
    }
}

// TODO: What does "extern 'system'" mean?
unsafe extern "system" fn window_proc(hwnd: HWND, u_msg: UINT, w_param: WPARAM, l_param: LPARAM) -> isize {
    DefWindowProcW(hwnd, u_msg, w_param, l_param)
}