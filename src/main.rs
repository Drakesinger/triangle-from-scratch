#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


#![allow(unused_imports)]
// #![allow(non_snake_case)]
// #![allow(dead_code)]
// #![allow(unused_macros)]
// #![allow(unreachable_code)]

use triangle_from_scratch::win32::*;
use core::ptr::{null, null_mut};

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


fn main() {
    println!("Hello, world!");

    let handle_instance = unsafe { GetModuleHandleW(core::ptr::null()) };
    let sample_window_class_wn = wide_null("Sample Window Class");
    let sample_window_name_wn = wide_null("Sample Window Name");

    let mut window_class: WNDCLASSW = WNDCLASSW::default();
    window_class.lpfnWndProc = Some(window_procedure);
    window_class.hInstance = handle_instance;
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
