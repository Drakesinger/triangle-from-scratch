#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused_imports)]
// #![allow(non_snake_case)]
// #![allow(dead_code)]
// #![allow(unused_macros)]
// #![allow(unreachable_code)]

use core::ptr::{null, null_mut};
use triangle_from_scratch::win32::*;

unsafe extern "system" fn window_procedure(
    hwnd: HWND,
    msg: UINT,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    match msg {
        WM_NCCREATE => {
            println!("WM_NCCREATE");

            let create_struct = l_param as *mut CREATESTRUCTW;
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
        _ => return DefWindowProcW(hwnd, msg, w_param, l_param),
    }

    0
}

/// Returns a handle to the file (executable file) used to create the calling process.
///
/// See : [`GetModuleHandleW`](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlew)
pub fn get_process_handle() -> HMODULE {
    // Safety : Null provides the executable handle that created the calling process.
    // See [MSDN - `GetModuleHandleW` Parameters}(https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlew#parameters)
    unsafe { GetModuleHandleW(core::ptr::null()) }
}

/// Predefined cursor styles.
pub enum EIDCursor {
    /// Standard arrow and small hourglass
    AppStarting = 32650,
    /// Standard arrow
    Arrow = 32512,
    /// Crosshair
    Cross = 32515,
    /// Hand
    Hand = 32649,
    /// Arrow and question mark
    Help = 32651,
    /// I-beam
    IBeam = 32513,
    /// Slashed circle
    No = 32648,
    /// Four-pointed arrow pointing north, south, east, and west
    SizeAll = 32646,
    /// Double-pointed arrow pointing northeast and southwest
    SizeNeSw = 32643,
    /// Double-pointed arrow pointing north and south
    SizeNS = 32645,
    /// Double-pointed arrow pointing northwest and southeast
    SizeNwSe = 32642,
    /// Double-pointed arrow pointing west and east
    SizeWE = 32644,
    /// Vertical arrow
    UpArrow = 32516,
    /// Hourglass
    Wait = 32514,
}

/// Loads the specified predefined cursors.
///
/// See : [`LoadCursorW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw)
pub fn load_predefined_cursor(cursor: EIDCursor) -> Result<HCURSOR, Win32Error> {
    let resource = MAKEINTRESOURCE(cursor as WORD);
    // Safety : The enum only allows values from the the approved cursors list.
    let hcursor = unsafe { LoadCursorW(null_mut(), resource) };
    if hcursor.is_null() {
        Err(get_last_error())
    } else {
        Ok(hcursor)
    }
}

#[derive(Debug)] // trait used for when you want to display info to a programmer
#[repr(transparent)]
pub struct Win32Error(pub DWORD);
impl core::fmt::Display for Win32Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dwFlags = FORMAT_MESSAGE_ALLOCATE_BUFFER
            | FORMAT_MESSAGE_FROM_SYSTEM
            | FORMAT_MESSAGE_IGNORE_INSERTS;
        let lpSource = null_mut();
        let dwMessageId = self.0; // NOTE is this equivalent to *this* ?
        let dwLanguageId = 0;

        // The buffer that is going to be alocated by FormatMessageW.
        let mut buffer: *mut u16 = null_mut();
        // Address where the pointer is located as LPTSTR (Long Pointer TCHAR string, in our case TCHAR is WCHAR, *mut u16)
        // In C : (LPTSTR)&buffer
        let lpBuffer = &mut buffer as *mut LPWSTR as *mut u16;
        let nSize = 0; // Minumum size of the buffer allocated.
        let Arguments = null_mut();
        let tchar_count_excluding_null = unsafe {
            FormatMessageW(
                dwFlags,
                lpSource,
                dwMessageId,
                dwLanguageId,
                lpBuffer,
                nSize,
                Arguments,
            )
        };

        if tchar_count_excluding_null == 0 || buffer.is_null() {
            return Err(core::fmt::Error);
        } else {
            todo!("We got something valid")
        }
    }
}

/// Registers a window class struct.
///
/// FIXME Partially wrapped
/// ## Safety
///
/// All pointer fields of the struct must be valid.
///
/// See [`RegisterClassW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassw)
pub unsafe fn register_class(window_class: &WNDCLASSW) -> Result<ATOM, Win32Error> {
    let atom = RegisterClassW(window_class);
    if atom == 0 {
        Err(get_last_error())
    } else {
        Ok(atom)
    }
}

/// Gets the thread-local last-error code value.
///
/// See [`GetLastError`](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror)
pub fn get_last_error() -> Win32Error {
    Win32Error(unsafe { GetLastError() })
}

/// Gets a message from the thread's message queue.
///
/// The message can be for any window in this thread,
/// or it can be a non-window message as well
pub fn get_any_message() -> Result<MSG, Win32Error> {
    let mut msg = MSG::default();
    let mut output = unsafe { GetMessageW(&mut msg, null_mut(), 0, 0) };
    if output == -1 {
        // We got an error.
        Err(get_last_error())
    } else {
        Ok(msg)
    }
}

fn main() {
    println!("Hello, world!");

    let handle_instance = get_process_handle();
    let sample_window_class_wn = wide_null("Sample Window Class");
    let sample_window_name_wn = wide_null("Sample Window Name");

    let mut window_class: WNDCLASSW = WNDCLASSW::default();
    window_class.lpfnWndProc = Some(window_procedure);
    window_class.hInstance = handle_instance;
    window_class.hCursor = load_predefined_cursor(EIDCursor::Arrow).unwrap();

    // We still need a LPCWSTR
    // a wide string, to Windows, means a UTF-16 string
    window_class.lpszClassName = sample_window_class_wn.as_ptr();
    let atom = unsafe { register_class(&window_class) }.unwrap_or_else(|()| {
        let last_error = unsafe { GetLastError() };
        panic!(
            "Could not register the window class, error code:{}",
            last_error
        );
    });

    // State passed to the window.
    let lp_param: *mut i32 = Box::leak(Box::new(5_i32));

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
            handle_instance,
            lp_param.cast(), //null_mut(),
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
