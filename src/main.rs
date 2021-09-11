#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_macros)]
#![allow(unreachable_code)]

/// Following the tutorial :
/// https://rust-tutorials.github.io/triangle-from-scratch/opening_a_window/win32.html

use core::ffi::c_void;
use core::ptr::{null, null_mut};
use std::os::raw::{c_int, c_uint};

// See
// - https://docs.microsoft.com/en-us/cpp/cpp/data-type-ranges?view=msvc-160
// - https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types
// Headers potential location (depends on SDK installed):
// C:\Program Files (x86)\Windows Kits\10\Include\10.0.19041.0\um\WinUser.h

// These are defined in std::os::raw.
// type c_int = i32;
// type c_uint = u32;

type HANDLE = PVOID;
type HINSTANCE = HANDLE;
type HICON = HANDLE;
type HCURSOR = HANDLE;
type HBRUSH = HANDLE;
type HWND = HANDLE;

type LPCWSTR = *const WCHAR;
type WCHAR = wchar_t;
type wchar_t = u16; // Wide char, 2 bytes.

type UINT = c_uint;
type UINT_PTR = usize;
type INT_PTR = isize;
type LONG_PTR = UINT_PTR;

type WPARAM = UINT_PTR;
type LPARAM = LONG_PTR;
type LRESULT = LONG_PTR;

type PVOID = *mut c_void;

type WNDPROC = Option<
    // WindowProc
    unsafe extern "system" fn(hwnd: HWND, uMsg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT,
>;

#[repr(C)] // Memory Layout : https://doc.rust-lang.org/reference/type-layout.html
///[`WNDCLASSW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-wndclassw)
pub struct WNDCLASSW {
    style: UINT,
    lpfnWndProc: WNDPROC,
    cbClsExtra: c_int,
    cbWndExtra: c_int,
    hInstance: HINSTANCE,
    hIcon: HICON,
    hCursor: HCURSOR,
    hbrBackground: HBRUSH,
    lpszMenuName: LPCWSTR,
    lpszClassName: LPCWSTR,
}

impl Default for WNDCLASSW {
    #[must_use]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

macro_rules! unsafe_impl_default_zeroed {
    ($t:ty) => {
        impl Default for $t {
            #[inline]
            #[must_use]
            fn default() -> Self {
                unsafe { core::mem::zeroed() }
            }
        }
    };
}

type HMODULE = HINSTANCE;
type DWORD = c_ulong;
type c_ulong = u32;

#[link(name = "Kernel32")]
extern "system" {
    /// [`GetModuleHandleW`](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlew)
    pub fn GetModuleHandleW(lpModuleName: LPCWSTR) -> HMODULE;

    /// [`GetLastError`](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror)
    pub fn GetLastError() -> DWORD;
}

type ATOM = WORD;
type WORD = c_ushort;
type c_ushort = u16;

type HMENU = HANDLE;
type LPVOID = *mut c_void;

type BOOL = c_int;

#[link(name = "User32")]
extern "system" {
    /// [`RegisterClassW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassw)
    pub fn RegisterClassW(lpWndClass: *const WNDCLASSW) -> ATOM;

    /// [`CreateWindowExW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw#syntax)
    pub fn CreateWindowExW(
        dwExStyle: DWORD,
        lpClassName: LPCWSTR,
        lpWindowName: LPCWSTR,
        dwStyle: DWORD,
        X: c_int,
        Y: c_int,
        nWidth: c_int,
        nHeight: c_int,
        hWndParent: HWND,
        hMenu: HMENU,
        hInstance: HINSTANCE,
        lpParam: LPVOID,
    ) -> HWND;

    /// [`ShowWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)
    pub fn ShowWindow(hWnd: HWND, nCmdShow: c_int) -> BOOL;

    /// [`DefWindowProcW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-defwindowprocw)
    pub fn DefWindowProcW(hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
}

/// Turns a Rust string slice into a null-terminated utf-16 vector.
pub fn wide_null(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}

/// Window Styles
const WS_OVERLAPPED: u32 = 0x00000000;
const WS_POPUP: u32 = 0x80000000;
const WS_CHILD: u32 = 0x40000000;
const WS_MINIMIZE: u32 = 0x20000000;
const WS_VISIBLE: u32 = 0x10000000;
const WS_DISABLED: u32 = 0x08000000;
const WS_CLIPSIBLINGS: u32 = 0x04000000;
const WS_CLIPCHILDREN: u32 = 0x02000000;
const WS_MAXIMIZE: u32 = 0x01000000;
const WS_CAPTION: u32 = 0x00C00000; /* WS_BORDER | WS_DLGFRAME  */
const WS_BORDER: u32 = 0x00800000;
const WS_DLGFRAME: u32 = 0x00400000;
const WS_VSCROLL: u32 = 0x00200000;
const WS_HSCROLL: u32 = 0x00100000;
const WS_SYSMENU: u32 = 0x00080000;
const WS_THICKFRAME: u32 = 0x00040000;
const WS_GROUP: u32 = 0x00020000;
const WS_TABSTOP: u32 = 0x00010000;

const WS_MINIMIZEBOX: u32 = 0x00020000;
const WS_MAXIMIZEBOX: u32 = 0x00010000;

const WS_TILED: u32 = WS_OVERLAPPED;
const WS_ICONIC: u32 = WS_MINIMIZE;
const WS_SIZEBOX: u32 = WS_THICKFRAME;
const WS_TILEDWINDOW: u32 = WS_OVERLAPPEDWINDOW;

/// Common Window Styles
const WS_OVERLAPPEDWINDOW: u32 =
    WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX;

const WS_POPUPWINDOW: u32 = WS_POPUP | WS_BORDER | WS_SYSMENU;

const WS_CHILDWINDOW: u32 = WS_CHILD;

const CW_USEDEFAULT: c_int = 0x80000000_u32 as c_int;

const SW_SHOW: c_int = 5;

fn main() {
    println!("Hello, world!");

    let hInstance = unsafe { GetModuleHandleW(core::ptr::null()) };
    let sample_window_class_wn = wide_null("Sample Window Class");
    let sample_window_name_wn = wide_null("Sample Window Name");

    let mut window_class: WNDCLASSW = WNDCLASSW::default();
    window_class.lpfnWndProc = Some(DefWindowProcW);
    window_class.hInstance = hInstance;

    // We still need a LPCWSTR
    // a wide string, to Windows, means a UTF-16 string
    window_class.lpszClassName = sample_window_class_wn.as_ptr();
    let atom = unsafe { RegisterClassW(&window_class) };
    if atom == 0 {
        let last_error = unsafe { GetLastError() };
        panic!(
            "Could not register the window class, error code:{}",
            last_error
        );
    }

    let window_handle = unsafe {
        CreateWindowExW(
            0,
            sample_window_class_wn.as_ptr(),
            sample_window_name_wn.as_ptr(),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            null_mut(),
            null_mut(),
            hInstance,
            null_mut(),
        )
    };
    if window_handle.is_null() {
        panic!("Failed to create a window");
    }

    let _previously_visible = unsafe { ShowWindow(window_handle, SW_SHOW) };
}
