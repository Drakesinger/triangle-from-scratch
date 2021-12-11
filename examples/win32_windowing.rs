#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_macros)]
#![allow(unused_imports)]
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
    // WindowProcedure
    unsafe extern "system" fn(hwnd: HWND, uMsg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT,
>;

/// Window Messages
const WM_NULL: u32 = 0x0000;
const WM_NCCREATE: u32 = 0x0081;
const WM_CREATE: u32 = 0x0001;
const WM_DESTROY: u32 = 0x0002;
const WM_MOVE: u32 = 0x0003;
const WM_SIZE: u32 = 0x0005;

const WM_ACTIVATE: u32 = 0x0006;

const WM_SETFOCUS: u32 = 0x0007;
const WM_KILLFOCUS: u32 = 0x0008;
const WM_ENABLE: u32 = 0x000A;
const WM_SETREDRAW: u32 = 0x000B;
const WM_SETTEXT: u32 = 0x000C;
const WM_GETTEXT: u32 = 0x000D;
const WM_GETTEXTLENGTH: u32 = 0x000E;
const WM_PAINT: u32 = 0x000F;
const WM_CLOSE: u32 = 0x0010;
const WM_QUIT: u32 = 0x0012;
const WM_ERASEBKGND: u32 = 0x0014;
const WM_SYSCOLORCHANGE: u32 = 0x0015;
const WM_SHOWWINDOW: u32 = 0x0018;
const WM_WININICHANGE: u32 = 0x001A;
const WM_DEVMODECHANGE: u32 = 0x001B;
const WM_ACTIVATEAPP: u32 = 0x001C;
const WM_FONTCHANGE: u32 = 0x001D;
const WM_TIMECHANGE: u32 = 0x001E;
const WM_CANCELMODE: u32 = 0x001F;
const WM_SETCURSOR: u32 = 0x0020;
const WM_MOUSEACTIVATE: u32 = 0x0021;
const WM_CHILDACTIVATE: u32 = 0x0022;
const WM_QUEUESYNC: u32 = 0x0023;
const WM_GETMINMAXINFO: u32 = 0x0024;

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

const IDC_ARROW: LPCWSTR = MAKEINTRESOURCE(32512);

const COLOR_WINDOW: u32 = 5;

const MB_OKCANCEL: u32 = 0x00000001;
const IDOK: c_int = 1;

const GWLP_USERDATA: c_int = -21;

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

type LONG = c_long;
type c_long = i32;

#[repr(C)]
pub struct POINT {
    x: LONG,
    y: LONG,
}
unsafe_impl_default_zeroed!(POINT);

#[repr(C)]
pub struct MSG {
    hwnd: HWND,
    message: UINT,
    wParam: WPARAM,
    lParam: LPARAM,
    time: DWORD,
    pt: POINT,
    lPrivate: DWORD,
}
unsafe_impl_default_zeroed!(MSG);

type HMODULE = HINSTANCE;
type DWORD = c_ulong;
type c_ulong = u32;

type HDC = HANDLE;
type BYTE = u8;

#[repr(C)]
pub struct RECT {
    left: LONG,
    top: LONG,
    right: LONG,
    bottom: LONG,
}
unsafe_impl_default_zeroed!(RECT);

#[repr(C)]
pub struct PAINTSTRUCT {
    hdc: HDC,
    fErase: BOOL,
    rcPaint: RECT,
    fRestore: BOOL,
    fIncUpdate: BOOL,
    rgbReserved: [BYTE; 32],
}
unsafe_impl_default_zeroed!(PAINTSTRUCT);

#[repr(C)]
pub struct CREATESTRUCTW {
    lpCreateParams: LPVOID,
    hInstance: HINSTANCE,
    hMenu: HMENU,
    hwndParent: HWND,
    cy: c_int,
    cx: c_int,
    y: c_int,
    x: c_int,
    style: LONG,
    lpszName: LPCWSTR,
    lpszClass: LPCWSTR,
    dwExStyle: DWORD,
}
unsafe_impl_default_zeroed!(CREATESTRUCTW);

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
type LPMSG = *const MSG;

type LPWSTR = *mut WCHAR;
type ULONG_PTR = usize;

pub const fn MAKEINTRESOURCE(i: WORD) -> LPWSTR {
    i as ULONG_PTR as LPWSTR
}

// type LPPAINTSTRUCT = *mut PAINTSTRUCT;

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

    ///[`GetMessageW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmessagew)
    pub fn GetMessageW(lpMsg: LPMSG, hWnd: HWND, wMsgFilterMin: UINT, wMsgFilterMax: UINT) -> BOOL;

    /// [`TranslateMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-translatemessage)
    pub fn TranslateMessage(lpMsg: *const MSG) -> BOOL;

    /// [`DispatchMessageW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-dispatchmessagew)
    pub fn DispatchMessageW(lpMsg: *const MSG) -> LRESULT;

    /// [`PostQuitMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-postquitmessage)
    pub fn PostQuitMessage(nExitCode: c_int);

    /// [`DestroyWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroywindow)
    pub fn DestroyWindow(hWnd: HWND) -> BOOL;

    /// [`LoadCursorW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw)
    pub fn LoadCursorW(hInstance: HINSTANCE, lpCursorName: LPCWSTR) -> HCURSOR;

    /// [`BeginPaint`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-beginpaint)
    pub fn BeginPaint(hWnd: HWND, lpPaint: *mut PAINTSTRUCT) -> HDC;

    /// [`FillRect`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-fillrect)
    pub fn FillRect(hDC: HDC, lprc: *const RECT, hbr: HBRUSH) -> c_int;

    /// [`EndPaint`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-endpaint)
    pub fn EndPaint(hWnd: HWND, lpPaint: *const PAINTSTRUCT) -> BOOL;

    /// [`MessageBoxW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-messageboxw)
    pub fn MessageBoxW(hWnd: HWND, lpText: LPCWSTR, lpCaption: LPCWSTR, uType: UINT) -> c_int;

    /// [`SetWindowLongPtrW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowlongptrw)
    pub fn SetWindowLongPtrW(hWnd: HWND, nIndex: c_int, dwNewLong: LONG_PTR) -> LONG_PTR;

    /// [`GetWindowLongPtrW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowlongptrw)
    pub fn GetWindowLongPtrW(hWnd: HWND, nIndex: c_int) -> LONG_PTR;

}

unsafe extern "system" fn window_procedure(
    hwnd: HWND,
    uMsg: UINT,
    wParam: WPARAM,
    lParam: LPARAM,
) -> LRESULT {
    match uMsg {
        WM_NCCREATE => {
            println!("WM_NCCREATE");

            let create_struct = lParam as *mut CREATESTRUCTW;
            if create_struct.is_null() {
                println!("WTF");
                return 0;
            }
            let boxed_i32_ptr: *mut i32 = (*create_struct).lpCreateParams.cast();
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, boxed_i32_ptr as LONG_PTR);
            return 1;
        }
        WM_CREATE => {
            println!("WM_CREATE");
            return 0;
        }

        WM_PAINT => {
            let mut ps: PAINTSTRUCT = PAINTSTRUCT::default();
            let hdc: HDC = BeginPaint(hwnd, &mut ps);

            // All painting occurs here, between BeginPaint and EndPaint.
            let ptr_to_user_data = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut i32;
            println!("Current ptr_to_user_data value: {}", *ptr_to_user_data);
            *ptr_to_user_data += 1;
            let _success = FillRect(hdc, &ps.rcPaint, (COLOR_WINDOW + 2) as HBRUSH);
            EndPaint(hwnd, &ps);
        }

        // We do not specifically need to treat these, we could let windows do the heavy lifting.
        WM_CLOSE => {
            // Extra stuff to show a message box.
            let message_box_text = wide_null("Do you really want to quit?");
            let message_box_caption = wide_null("Wait a minnute!");
            let user_input = MessageBoxW(
                hwnd,
                message_box_text.as_ptr(),
                message_box_caption.as_ptr(),
                MB_OKCANCEL,
            );
            if user_input == IDOK {
                DestroyWindow(hwnd);
            }

            return 0;
            // Otherwise
            // drop(DestroyWindow(hwnd));
        }
        WM_DESTROY => {
            // Perform cleanup.
            let ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut i32;
            Box::from_raw(ptr);
            println!("Cleaned up the box");
            PostQuitMessage(0)
        }
        _ => return DefWindowProcW(hwnd, uMsg, wParam, lParam),
    }

    0
}

/// Turns a Rust string slice into a null-terminated utf-16 vector.
pub fn wide_null(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}

fn main() {
    println!("Hello, world!");

    let hInstance = unsafe { GetModuleHandleW(core::ptr::null()) };
    let sample_window_class_wn = wide_null("Sample Window Class");
    let sample_window_name_wn = wide_null("Sample Window Name");

    let mut window_class: WNDCLASSW = WNDCLASSW::default();
    window_class.lpfnWndProc = Some(window_procedure);
    window_class.hInstance = hInstance;
    window_class.hCursor = unsafe { LoadCursorW(null_mut(), IDC_ARROW) };

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

    // State passed to the window.
    let lpParam: *mut i32 = Box::leak(Box::new(5_i32));

    // Now we create our window.
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
            lpParam.cast(), //null_mut(),
        )
    };
    if window_handle.is_null() {
        panic!("Failed to create a window");
    }

    let _previously_visible = unsafe { ShowWindow(window_handle, SW_SHOW) };

    let mut msg = MSG::default();
    loop {
        let message_return = unsafe { GetMessageW(&mut msg, null_mut(), 0, 0) };
        // If we receive the WM_QUIT message, the return value is 0.
        if message_return == 0 {
            break;
        }
        // If we receive a -1, then there was an error.
        else if message_return == -1 {
            let last_error = unsafe { GetLastError() };
            panic!(
                "Error when trying to get a message. Error code: {}",
                last_error
            );
        } else {
            unsafe {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
    }
}
