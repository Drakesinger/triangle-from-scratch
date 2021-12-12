#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

/// Following the tutorial :
/// https://rust-tutorials.github.io/triangle-from-scratch/opening_a_window/win32.html
use core::ffi::c_void;
use std::os::raw::{c_int, c_uint};

// See
// - https://docs.microsoft.com/en-us/cpp/cpp/data-type-ranges?view=msvc-160
// - https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types
// Headers potential location (depends on SDK installed):
// C:\Program Files (x86)\Windows Kits\10\Include\10.0.19041.0\um\WinUser.h

// These are defined in std::os::raw.
// type c_int = i32;
// type c_uint = u32;

pub type HANDLE = PVOID;
pub type HINSTANCE = HANDLE;
pub type HICON = HANDLE;
pub type HCURSOR = HANDLE;
pub type HBRUSH = HANDLE;
pub type HWND = HANDLE;

pub type LPCWSTR = *const WCHAR;
pub type WCHAR = wchar_t;
pub type wchar_t = u16; // Wide char, 2 bytes.

pub type UINT = c_uint;
pub type UINT_PTR = usize;
pub type INT_PTR = isize;
pub type LONG_PTR = UINT_PTR;

pub type WPARAM = UINT_PTR;
pub type LPARAM = LONG_PTR;
pub type LRESULT = LONG_PTR;

pub type PVOID = *mut c_void;

pub type LONG = c_long;
pub type c_long = i32;

pub type HMODULE = HINSTANCE;
pub type DWORD = c_ulong;
pub type c_ulong = u32;

/// A handle to the device context.
pub type HDC = HANDLE;
pub type BYTE = u8;

pub type ATOM = WORD;
pub type WORD = c_ushort;
pub type c_ushort = u16;

pub type HMENU = HANDLE;
pub type LPVOID = *mut c_void;

pub type BOOL = c_int;
pub type LPMSG = *const MSG;

pub type LPWSTR = *mut WCHAR;
pub type ULONG_PTR = usize;

pub type LPPAINTSTRUCT = *mut PAINTSTRUCT;

pub type LPCVOID = *const core::ffi::c_void;
pub type va_list = *mut c_char;
pub type c_char = i8;

pub type HLOCAL = HANDLE;

///[`WNDPROC`](https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms633573(v=vs.85))
/// This type defines a pointer to  the application-defined callback function `WindowProc`
/// that processes messages sent to a window.
pub type WNDPROC = Option<
    // WindowProcedure
    unsafe extern "system" fn(hwnd: HWND, uMsg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT,
>;

/// Window Messages
pub const WM_NULL: u32 = 0x0000;
pub const WM_NCCREATE: u32 = 0x0081;
pub const WM_CREATE: u32 = 0x0001;
pub const WM_DESTROY: u32 = 0x0002;
pub const WM_MOVE: u32 = 0x0003;
pub const WM_SIZE: u32 = 0x0005;

pub const WM_ACTIVATE: u32 = 0x0006;

pub const WM_SETFOCUS: u32 = 0x0007;
pub const WM_KILLFOCUS: u32 = 0x0008;
pub const WM_ENABLE: u32 = 0x000A;
pub const WM_SETREDRAW: u32 = 0x000B;
pub const WM_SETTEXT: u32 = 0x000C;
pub const WM_GETTEXT: u32 = 0x000D;
pub const WM_GETTEXTLENGTH: u32 = 0x000E;
pub const WM_PAINT: u32 = 0x000F;
pub const WM_CLOSE: u32 = 0x0010;
pub const WM_QUIT: u32 = 0x0012;
pub const WM_ERASEBKGND: u32 = 0x0014;
pub const WM_SYSCOLORCHANGE: u32 = 0x0015;
pub const WM_SHOWWINDOW: u32 = 0x0018;
pub const WM_WININICHANGE: u32 = 0x001A;
pub const WM_DEVMODECHANGE: u32 = 0x001B;
pub const WM_ACTIVATEAPP: u32 = 0x001C;
pub const WM_FONTCHANGE: u32 = 0x001D;
pub const WM_TIMECHANGE: u32 = 0x001E;
pub const WM_CANCELMODE: u32 = 0x001F;
pub const WM_SETCURSOR: u32 = 0x0020;
pub const WM_MOUSEACTIVATE: u32 = 0x0021;
pub const WM_CHILDACTIVATE: u32 = 0x0022;
pub const WM_QUEUESYNC: u32 = 0x0023;
pub const WM_GETMINMAXINFO: u32 = 0x0024;

/// Window Styles
pub const WS_OVERLAPPED: u32 = 0x00000000;
pub const WS_POPUP: u32 = 0x80000000;
pub const WS_CHILD: u32 = 0x40000000;
pub const WS_MINIMIZE: u32 = 0x20000000;
pub const WS_VISIBLE: u32 = 0x10000000;
pub const WS_DISABLED: u32 = 0x08000000;
pub const WS_CLIPSIBLINGS: u32 = 0x04000000;
pub const WS_CLIPCHILDREN: u32 = 0x02000000;
pub const WS_MAXIMIZE: u32 = 0x01000000;
pub const WS_CAPTION: u32 = 0x00C00000; /* WS_BORDER | WS_DLGFRAME  */
pub const WS_BORDER: u32 = 0x00800000;
pub const WS_DLGFRAME: u32 = 0x00400000;
pub const WS_VSCROLL: u32 = 0x00200000;
pub const WS_HSCROLL: u32 = 0x00100000;
pub const WS_SYSMENU: u32 = 0x00080000;
pub const WS_THICKFRAME: u32 = 0x00040000;
pub const WS_GROUP: u32 = 0x00020000;
pub const WS_TABSTOP: u32 = 0x00010000;

pub const WS_MINIMIZEBOX: u32 = 0x00020000;
pub const WS_MAXIMIZEBOX: u32 = 0x00010000;

pub const WS_TILED: u32 = WS_OVERLAPPED;
pub const WS_ICONIC: u32 = WS_MINIMIZE;
pub const WS_SIZEBOX: u32 = WS_THICKFRAME;
pub const WS_TILEDWINDOW: u32 = WS_OVERLAPPEDWINDOW;

/// Common Window Styles
pub const WS_OVERLAPPEDWINDOW: u32 =
    WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX;

pub const WS_POPUPWINDOW: u32 = WS_POPUP | WS_BORDER | WS_SYSMENU;

pub const WS_CHILDWINDOW: u32 = WS_CHILD;

pub const CW_USEDEFAULT: c_int = 0x80000000_u32 as c_int;

pub const SW_SHOW: c_int = 5;

pub const IDC_ARROW: LPCWSTR = MAKEINTRESOURCE(32512);

/// See [`GetSysColor`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor)
pub enum SysColor {
    _3D_DARK_SHADOW = 21,
    _3D_LIGHT = 22,
    ACTIVE_BORDER = 10,
    ACTIVE_CAPTION = 2,
    APP_WORKSPACE = 12,
    /// Button face, also "3D face" color.
    BUTTON_FACE = 15,
    /// Button highlight, also "3D highlight" color.
    BUTTON_HIGHLIGHT = 20,
    /// Button shadow, also "3D shadow" color.
    BUTTON_SHADOW = 16,
    BUTTON_TEXT = 18,
    CAPTION_TEXT = 9,
    /// Desktop background color
    DESKTOP = 1,
    GRADIENT_ACTIVE_CAPTION = 27,
    GRADIENT_INACTIVE_CAPTION = 28,
    GRAY_TEXT = 17,
    HIGHLIGHT = 13,
    HIGHLIGHT_TEXT = 14,
    HOT_LIGHT = 26,
    INACTIVE_BORDER = 11,
    INACTIVE_CAPTION = 3,
    INACTIVE_CAPTION_TEXT = 19,
    INFO_BACKGROUND = 24,
    INFO_TEXT = 23,
    MENU = 4,
    MENU_HIGHLIGHT = 29,
    MENU_BAR = 30,
    MENU_TEXT = 7,
    SCROLL_BAR = 0,
    WINDOW = 5,
    WINDOW_FRAME = 6,
    WINDOW_TEXT = 8,
}

pub const MB_OKCANCEL: u32 = 0x00000001;
pub const IDOK: c_int = 1;

pub const GWLP_USERDATA: c_int = -21;

pub const FORMAT_MESSAGE_ALLOCATE_BUFFER: u32 = 0x00000100;
pub const FORMAT_MESSAGE_FROM_SYSTEM: u32 = 0x00001000;
pub const FORMAT_MESSAGE_IGNORE_INSERTS: u32 = 0x00000200;

/// Redraws the entire window if a movement or size adjustment changes the height of the client area.
/// 
/// See [Window Class Styles - Constants](https://docs.microsoft.com/en-us/windows/win32/winmsg/window-class-styles#constants)
pub const CS_VREDRAW: u32 = 0x0001;
/// Redraws the entire window if a movement or size adjustment changes the width of the client area.
/// 
/// See [Window Class Styles - Constants](https://docs.microsoft.com/en-us/windows/win32/winmsg/window-class-styles#constants)
pub const CS_HREDRAW: u32 = 0x0002;
/// Sends a double-click message to the window procedure when the user double-clicks the mouse while
/// the cursor is within a window belonging to the class.
/// 
/// See [Window Class Styles - Constants](https://docs.microsoft.com/en-us/windows/win32/winmsg/window-class-styles#constants)
pub const CS_DBLCLKS: u32 = 0x0008;
/// Allocates a unique device context for each window in the class.
/// See [Window Class Styles - Constants](https://docs.microsoft.com/en-us/windows/win32/winmsg/window-class-styles#constants)
pub const CS_OWNDC: u32 = 0x0020;
/// Allocates one device context to be shared by all windows in the class. 
/// Because window classes are process specific, it is possible for multiple threads of an 
/// application to create a window of the same class. It is also possible for the threads to attempt
/// to use the device context simultaneously. When this happens, the system allows only one thread
/// to successfully finish its drawing operation.
/// 
/// See [Window Class Styles - Constants](https://docs.microsoft.com/en-us/windows/win32/winmsg/window-class-styles#constants)
pub const CS_CLASSDC: u32 = 0x0040;
/// Sets the clipping rectangle of the child window to that of the parent window so that the child
/// can draw on the parent. A window with the CS_PARENTDC style bit receives a regular device context
/// from the system's cache of device contexts. It does not give the child the parent's device 
/// context or device context settings. Specifying CS_PARENTDC enhances an application's performance.
/// 
/// See [Window Class Styles - Constants](https://docs.microsoft.com/en-us/windows/win32/winmsg/window-class-styles#constants)
pub const CS_PARENTDC: u32 = 0x0080;
/// Disables Close on the window menu.
/// 
/// See [Window Class Styles - Constants](https://docs.microsoft.com/en-us/windows/win32/winmsg/window-class-styles#constants)
pub const CS_NOCLOSE: u32 = 0x0200;
/// Saves, as a bitmap, the portion of the screen image obscured by a window of this class.
/// When the window is removed, the system uses the saved bitmap to restore the screen image, 
/// including other windows that were obscured. Therefore, the system does not send `WM_PAINT` 
/// messages to windows that were obscured if the memory used by the bitmap has not been discarded
/// and if other screen actions have not invalidated the stored image.
/// This style is useful for small windows (for example, menus or dialog boxes) that are displayed
/// briefly and then removed before other screen activity takes place. This style increases the 
/// time required to display the window, because the system must first allocate memory to store
/// the bitmap.
/// 
/// See [Window Class Styles - Constants](https://docs.microsoft.com/en-us/windows/win32/winmsg/window-class-styles#constants)
pub const CS_SAVEBITS: u32 = 0x0800;
/// Aligns the window's client area on a byte boundary (in the x direction).
/// This style affects the width of the window and its horizontal placement on the display.
/// 
/// See [Window Class Styles - Constants](https://docs.microsoft.com/en-us/windows/win32/winmsg/window-class-styles#constants)
pub const CS_BYTEALIGNCLIENT: u32 = 0x1000;
/// Aligns the window on a byte boundary (in the x direction).
/// This style affects the width of the window and its horizontal placement on the display.
/// 
/// See [Window Class Styles - Constants](https://docs.microsoft.com/en-us/windows/win32/winmsg/window-class-styles#constants)
pub const CS_BYTEALIGNWINDOW: u32 = 0x2000;
/// Indicates that the window class is an application global class.
/// For more information, see the ["Application Global Classes"](https://docs.microsoft.com/en-us/windows/win32/winmsg/about-window-classes#application-global-classes) section of About Window Classes.
/// 
/// See [Window Class Styles - Constants](https://docs.microsoft.com/en-us/windows/win32/winmsg/window-class-styles#constants)
pub const CS_GLOBALCLASS: u32 = 0x4000;

#[repr(C)] // Memory Layout : https://doc.rust-lang.org/reference/type-layout.html
///[`WNDCLASSW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-wndclassw)
pub struct WNDCLASSW {
    /// The class style(s). This member can be any combination of the [Class Styles](https://docs.microsoft.com/en-us/windows/win32/winmsg/about-window-classes).
    pub style: UINT,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: c_int,
    pub cbWndExtra: c_int,
    pub hInstance: HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: HBRUSH,
    pub lpszMenuName: LPCWSTR,
    pub lpszClassName: LPCWSTR,
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

#[repr(C)]
pub struct POINT {
    pub x: LONG,
    pub y: LONG,
}
unsafe_impl_default_zeroed!(POINT);

#[repr(C)]
pub struct MSG {
    pub hwnd: HWND,
    pub message: UINT,
    pub wParam: WPARAM,
    pub lParam: LPARAM,
    pub time: DWORD,
    pub pt: POINT,
    pub lPrivate: DWORD,
}
unsafe_impl_default_zeroed!(MSG);

/// A pointer to a RECT structure that contains the logical coordinates of the rectangle to be filled
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RECT {
    pub left: LONG,
    pub top: LONG,
    pub right: LONG,
    pub bottom: LONG,
}
unsafe_impl_default_zeroed!(RECT);

/// See [`PAINTSTRUCT`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-paintstruct)
#[repr(C)]
pub struct PAINTSTRUCT {
    pub hdc: HDC,
    /// Indicates whether the background must be erased.
    ///
    /// This value is nonzero if the application should erase the background.
    /// The application is responsible for erasing the background if a window
    /// class is created without a background brush. For more information,
    /// see the description of the hbrBackground member of the
    /// [`WNDCLASS`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-wndclassw)
    /// structure.
    pub fErase: BOOL,
    pub rcPaint: RECT,
    pub fRestore: BOOL,
    pub fIncUpdate: BOOL,
    pub rgbReserved: [BYTE; 32],
}
unsafe_impl_default_zeroed!(PAINTSTRUCT);

#[repr(C)]
pub struct CREATESTRUCTW {
    pub lpCreateParams: LPVOID,
    pub hInstance: HINSTANCE,
    pub hMenu: HMENU,
    pub hwndParent: HWND,
    pub cy: c_int,
    pub cx: c_int,
    pub y: c_int,
    pub x: c_int,
    pub style: LONG,
    pub lpszName: LPCWSTR,
    pub lpszClass: LPCWSTR,
    pub dwExStyle: DWORD,
}
unsafe_impl_default_zeroed!(CREATESTRUCTW);

#[link(name = "Kernel32")]
extern "system" {
    /// [`GetModuleHandleW`](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlew)
    pub fn GetModuleHandleW(lpModuleName: LPCWSTR) -> HMODULE;

    /// [`GetLastError`](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror)
    pub fn GetLastError() -> DWORD;

    /// [`FormatMessageW`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew)
    pub fn FormatMessageW(
        dwFlags: DWORD,
        lpSource: LPCVOID,
        dwMessageId: DWORD,
        dwLanguageId: DWORD,
        lpBuffer: LPWSTR,
        nSize: DWORD,
        Arguments: *mut va_list,
    ) -> DWORD;

    /// [`LocalFree`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localfree)
    pub fn LocalFree(hMem: HLOCAL) -> HLOCAL;

    /// [`SetLastError`](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-setlasterror)
    pub fn SetLastError(dwErrCode: DWORD);
}

pub const fn MAKEINTRESOURCE(i: WORD) -> LPWSTR {
    i as ULONG_PTR as LPWSTR
}

/// See `C:\Program Files (x86)\Windows Kits\10\Include\10.0.19041.0\um\WinUser.h`
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

    /// [`GetSysColor `](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor)
    pub fn GetSysColor(nIndex: c_int) -> DWORD;

}

/// Turns a Rust string slice into a null-terminated utf-16 vector.
pub fn wide_null(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}
