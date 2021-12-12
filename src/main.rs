#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused_imports)]
#![allow(non_snake_case)]
// #![allow(dead_code)]
// #![allow(unused_macros)]
// #![allow(unreachable_code)]

use core::ptr::{null, null_mut};
use std::os::raw::{c_int, c_uint};
use triangle_from_scratch::win32::*;

/// Sets the thread-local last-error code value.
///
/// See [`SetLastError`](https://docs.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-setlasterror)
pub fn set_last_error(e: Win32Error) {
    unsafe { SetLastError(e.0) }
}

/// Sets the "userdata" pointer of the window (`GWLP_USERDATA`).
///
/// **Returns:** The previous userdata pointer.
///
/// [`SetWindowLongPtrW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowlongptrw)
pub fn set_window_userdata<T>(hwnd: HWND, ptr: *mut T) -> Result<*mut T, Win32Error> {
    set_last_error(Win32Error(0));
    let out = unsafe { SetWindowLongPtrW(hwnd, GWLP_USERDATA, ptr as LONG_PTR) };
    if out == 0 {
        // Check the last error, if it's also 0 then this is not a "real" error.
        let last_error = get_last_error();
        if last_error.0 == 0 {
            Ok(out as *mut T)
        } else {
            Err(last_error)
        }
    } else {
        Ok(out as *mut T)
    }
}

/// Gets the "userdata" pointer of the window (`GWLP_USERDATA`).
///
/// **Returns:** The userdata pointer.
///
/// [`GetWindowLongPtrW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowlongptrw)
pub fn get_window_userdata<T>(hwnd: HWND) -> Result<*mut T, Win32Error> {
    set_last_error(Win32Error(0));
    let out = unsafe { GetWindowLongPtrW(hwnd, GWLP_USERDATA) };
    if out == 0 {
        // if output is 0, it's only a "real" error if the last_error is non-zero
        let last_error = get_last_error();
        if last_error.0 != 0 {
            Err(last_error)
        } else {
            Ok(out as *mut T)
        }
    } else {
        Ok(out as *mut T)
    }
}

/// Indicates to the system that a thread has made a request to terminate (quit).
/// It is typically used in response to a [WM_DESTROY](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-destroy)
/// message.
///
/// The PostQuitMessage function posts a [WM_QUIT](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-quit) message
/// to the thread's message queue and returns immediately; the function simply indicates to the system that the thread
/// is requesting to quit at some time in the future.
///
/// When the thread retrieves the [WM_QUIT](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-quit) message from
/// its message queue, it should exit its message loop and return control to the system.
/// The exit value returned to the system **must** be the wParam parameter of the [WM_QUIT](https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-quit)
/// message.
///
/// See [`PostQuitMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-postquitmessage)
pub fn post_quit_message(exit_code: c_int) {
    unsafe { PostQuitMessage(exit_code) }
}

/// See [`BeginPaint`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-beginpaint)
pub fn begin_paint(hwnd: HWND) -> Result<(HDC, PAINTSTRUCT), Win32Error> {
    let mut ps: PAINTSTRUCT = PAINTSTRUCT::default();
    let hdc: HDC = unsafe { BeginPaint(hwnd, &mut ps) };
    if hdc.is_null() {
        Err(get_last_error())
    } else {
        Ok((hdc, ps))
    }
}

/// See [`EndPaint`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-endpaint)
pub fn end_paint(hwnd: HWND, ps: &PAINTSTRUCT) {
    // We do not care about the return value of EndPaint as it's always non-zero.
    unsafe { EndPaint(hwnd, ps) };
}

/// Fills a rectangle area with a specific color.
pub unsafe fn fill_rect_with_system_color(
    hdc: HDC,
    rect: &RECT,
    color: SysColor,
) -> Result<(), ()> {
    let _success = FillRect(hdc, rect, (color as u32 + 1) as HBRUSH);
    if _success != 0 {
        Ok(())
    } else {
        Err(())
    }
}

pub unsafe fn do_some_painting<F>(hwnd: HWND, f: F) -> Result<(), Win32Error>
where
    F: FnOnce(HDC, bool, RECT) -> Result<(), Win32Error>,
{
    let (hdc, ps) = begin_paint(hwnd)?;
    let output = f(hdc, ps.fErase != 0, ps.rcPaint);
    end_paint(hwnd, &ps);
    output
}

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
            let boxed_i32_ptr: *mut i32 = (*create_struct).lpCreateParams.cast::<i32>();
            if let Err(e) = set_window_userdata::<i32>(hwnd, boxed_i32_ptr) {
                println!("Couldn't set the WindowData pointer: {}", e);
                return 0;
            }
            return DefWindowProcW(hwnd, msg, w_param, l_param);
        }
        WM_CREATE => {
            println!("WM_CREATE");
            return 0;
        }

        WM_PAINT => {
            let get_userdata_result = get_window_userdata::<i32>(hwnd);
            match get_userdata_result {
                Ok(ptr_to_user_data) => {
                    if !ptr_to_user_data.is_null() {
                        println!("Current window userdata : {}", *ptr_to_user_data);
                        *ptr_to_user_data += 1;
                    } else {
                        println!("Userdata is empty.");
                    }
                }
                Err(e) => {
                    println!("Error while getting userdata: {}", e);
                }
            }

            // Either like this, or the below with closure.
            // let begin_paint_result = begin_paint(hwnd);
            // match begin_paint_result {
            //     Ok((hdc, ps)) => {
            //         // All painting occurs here, between BeginPaint and EndPaint.
            //         let _ = fill_rect_with_system_color(hdc, &ps.rcPaint, SysColor::WINDOW);
            //         end_paint(hwnd, &ps);
            //     }
            //     Err(e) => {
            //         println!("Error while trying to start painting: {}", e);
            //     }
            // }

            // I don't like this closure way too much, probably because it doesn't make sense for me too much yet.
            do_some_painting(hwnd, |hdc, _erase_bg, target_rect| {
                let _ = fill_rect_with_system_color(hdc, &target_rect, SysColor::WINDOW);
                Ok(())
            })
            .unwrap_or_else(|e| println!("Error while painting: {}", e));
        }

        // We do not specifically need to treat these, we could let windows do the heavy lifting.
        WM_CLOSE => {
            // Extra stuff to show a message box.
            let show_message_result =
                show_message_box(hwnd, "Wait a minute!", "Do you really want to quit?");
            match show_message_result {
                Ok(user_decision) => {
                    if user_decision == IDOK {
                        //DestroyWindow(hwnd);
                        // Otherwise
                        drop(DestroyWindow(hwnd));
                    }
                }
                Err(e) => {
                    println!("Error when showing the message box: {}", e);
                }
            }

            return 0;
        }
        WM_DESTROY => {
            // Perform cleanup.
            match get_window_userdata::<i32>(hwnd) {
                Ok(ptr) if !ptr.is_null() => {
                    Box::from_raw(ptr);
                    println!("Cleaned up the box");
                }
                Ok(_) => {
                    println!("Userdata pointer is null, no cleanup.");
                }
                Err(e) => {
                    println!(
                        "Error while getting the userdata to perform cleanup : {}",
                        e
                    );
                }
            }
            post_quit_message(0);
        }
        // This should be the actual return value.
        _ => return DefWindowProcW(hwnd, msg, w_param, l_param),
    }
    // Return 0, but we should never get here.
    0
}

pub fn show_message_box(hwnd: HWND, caption: &str, text: &str) -> Result<i32, Win32Error> {
    let message_box_text = wide_null(text);
    let message_box_caption = wide_null(caption);
    let message_result = unsafe {
        MessageBoxW(
            hwnd,
            message_box_text.as_ptr(),
            message_box_caption.as_ptr(),
            MB_OKCANCEL,
        )
    };
    if message_result == 0 {
        Err(get_last_error())
    } else {
        Ok(message_result)
    }
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

struct OnDropLocalFree(HLOCAL);
impl Drop for OnDropLocalFree {
    fn drop(&mut self) {
        unsafe { LocalFree(self.0) };
    }
}

#[derive(Debug)] // trait used for when you want to display info to a programmer
#[repr(transparent)]
pub struct Win32Error(pub DWORD);
impl core::fmt::Display for Win32Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // If the 29th bit is set, it's an error.
        if self.0 & (1 << 29) > 0 {
            return write!(f, "Win32ApplicationError({})", self.0);
        }

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
            // Wrap the buffer in the OnDropLocalFree struct so that when it goes
            // out of scope it gets dropped via a LocalFree call.
            let _on_drop = OnDropLocalFree(buffer as HLOCAL);
            // Binding it to a _ variable (ignored result) would drop it immediately.
            // There's been no error, let's access the buffer.
            let buffer_slice: &[u16] = unsafe {
                core::slice::from_raw_parts_mut(buffer, tchar_count_excluding_null as usize)
            };

            for decode_result in core::char::decode_utf16(buffer_slice.iter().copied()) {
                match decode_result {
                    Ok('\r') | Ok('\n') => write!(f, " ")?,
                    Ok(ch) => write!(f, "{}", ch)?,
                    Err(_) => write!(f, "�")?,
                }
            }

            Ok(())
        }
    }
}
impl std::error::Error for Win32Error {}

/// Registers a window class struct.
///
/// # ! Partially wrapped !
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
/// or it can be a non-window message as well.
///
/// See [`GetMessageW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmessagew)
#[inline(always)]
pub fn get_any_message() -> Result<MSG, Win32Error> {
    let mut msg = MSG::default();
    let output = unsafe { GetMessageW(&mut msg, null_mut(), 0, 0) };
    if output == -1 {
        // We got an error.
        Err(get_last_error())
    } else {
        Ok(msg)
    }
}

/// Creates a window.
///
/// * The window is not initially shown, you must call [`ShowWindow`] yourself.
///
/// See [`CreateWindowExW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw)
pub unsafe fn create_app_window(
    class_name: &str,
    window_name: &str,
    coordinates: Option<[i32; 2]>,
    [width, height]: [i32; 2],
    param: LPVOID,
) -> Result<HWND, Win32Error> {
    let class_name_wn = wide_null(class_name).as_ptr();
    let window_name_wn = wide_null(window_name).as_ptr();
    let position = match coordinates {
        Some([x, y]) => (x, y),
        None => (CW_USEDEFAULT, CW_USEDEFAULT),
    };

    let handle: HWND = CreateWindowExW(
        0,
        class_name_wn,
        window_name_wn,
        WS_OVERLAPPEDWINDOW,
        position.0,
        position.1,
        width,
        height,
        null_mut(),
        null_mut(),
        get_process_handle(),
        param,
    );
    if handle.is_null() {
        Err(get_last_error())
    } else {
        Ok(handle)
    }
}

/// Translates virtual-key messages into character messages.
///
/// The character messages are posted to the calling thread's message queue, to be read
/// the next time the thread calls the [`GetMessageW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmessagew)
/// or [PeekMessageW](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-peekmessagew) function.
///
/// See [`TranslateMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-translatemessage)
pub fn translate_message(msg: &MSG) -> bool {
    0 != unsafe { TranslateMessage(msg) }
}

fn main() {
    let window_name = "Sample Window Name";
    let class_name = "Sample Window Class";

    let handle_instance = get_process_handle();
    let sample_window_class_wn = wide_null(class_name);

    let mut window_class: WNDCLASSW = WNDCLASSW::default();
    window_class.lpfnWndProc = Some(window_procedure);
    window_class.hInstance = handle_instance;
    window_class.hCursor = load_predefined_cursor(EIDCursor::Arrow).unwrap();
    window_class.style = CS_OWNDC | CS_HREDRAW | CS_VREDRAW;

    // We still need a LPCWSTR
    // a wide string, to Windows, means a UTF-16 string
    window_class.lpszClassName = sample_window_class_wn.as_ptr();
    let _atom = unsafe { register_class(&window_class) }.unwrap_or_else(|e: Win32Error| {
        panic!("Could not register the window class, error code:{}", e);
    });

    // State passed to the window.
    let lp_param: *mut i32 = Box::leak(Box::new(5_i32));

    // Now we create our window.
    let window_handle =
        unsafe { create_app_window(class_name, window_name, None, [800, 600], lp_param.cast()) }
            .unwrap_or_else(|e: Win32Error| {
                panic!("Failed to create a window: {}", e);
            });

    let _previously_visible = unsafe { ShowWindow(window_handle, SW_SHOW) };

    loop {
        match get_any_message() {
            Ok(msg) => {
                if msg.message == WM_QUIT {
                    std::process::exit(msg.wParam as i32);
                } else {
                    translate_message(&msg);
                    unsafe {
                        DispatchMessageW(&msg);
                    }
                }
            }
            Err(e) => panic!("Error when getting a message from the queue: {}", e),
        }
    }
}
