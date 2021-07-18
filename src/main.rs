use bindings::Windows::Win32::Foundation::*;
use bindings::Windows::Win32::Graphics::Gdi::*;
use bindings::Windows::Win32::System::LibraryLoader::*;
use bindings::Windows::Win32::UI::WindowsAndMessaging::*;

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match message {
        // WM_CLOSE => {
        // Calls DestroyWindow by default
        // }

        // Called after window is destroyed
        WM_DESTROY => {
            // Posts WM_QUIT to the message queue
            unsafe { PostQuitMessage(0) };
            LRESULT::default()
        }

        WM_SIZE => LRESULT::default(),

        WM_PAINT => unsafe {
            static mut OPERATION: ROP_CODE = BLACKNESS;
            if OPERATION == BLACKNESS {
                OPERATION = WHITENESS;
            } else {
                OPERATION = BLACKNESS;
            }
            let mut paint_info: PAINTSTRUCT = Default::default();
            let dc = BeginPaint(window, &mut paint_info);
            let RECT {
                top,
                bottom,
                left,
                right,
            } = paint_info.rcPaint;
            PatBlt(dc, left, top, right - left, bottom - top, OPERATION);
            EndPaint(window, &paint_info);
            LRESULT::default()
        },
        _ => unsafe { DefWindowProcA(window, message, wparam, lparam) },
    }
}

fn main() -> windows::Result<()> {
    let instance = unsafe { GetModuleHandleA(None) };

    let wc = WNDCLASSEXA {
        cbSize: std::mem::size_of::<WNDCLASSEXA>() as u32,
        style: CS_HREDRAW | CS_VREDRAW | CS_OWNDC,
        lpfnWndProc: Some(wndproc),
        hInstance: instance,
        hCursor: unsafe { LoadCursorW(None, IDC_ARROW) },
        lpszClassName: PSTR(b"RustWindowClass\0".as_ptr() as _),
        ..Default::default()
    };

    unsafe {
        RegisterClassExA(&wc);

        let hwnd = CreateWindowExA(
            Default::default(),
            "RustWindowClass",
            "Hello Windows",
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            800,
            600,
            None, // no parent window
            None, // no menus
            instance,
            &mut 0 as *mut _ as _,
        );

        ShowWindow(hwnd, SW_SHOW)
    };

    loop {
        let mut message = MSG::default();

        if unsafe { GetMessageA(&mut message, None, 0, 0) }.into() {
            unsafe {
                TranslateMessage(&message);
                DispatchMessageA(&message);
            }
        } else {
            break;
        }
    }

    Ok(())
}
