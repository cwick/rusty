use bindings::Windows::Win32::Foundation::*;
use bindings::Windows::Win32::Graphics::Gdi::*;
use bindings::Windows::Win32::System::LibraryLoader::*;
use bindings::Windows::Win32::UI::WindowsAndMessaging::*;

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match message {
        // WM_CLOSE => {
        // Calls DestroyWindow by default
        // }

        // WM_CREATE => LRESULT::default(),

        // Called after window is destroyed
        WM_DESTROY => {
            // Posts WM_QUIT to the message queue
            unsafe { PostQuitMessage(0) };
            LRESULT::default()
        }

        // WM_SIZE => LRESULT::default(),
        WM_PAINT => {
            let bytes_per_pixel = 4;
            let mut paint_info: PAINTSTRUCT = Default::default();
            let device_context = unsafe { BeginPaint(window, &mut paint_info) };
            let mut window_rect = RECT::default();
            unsafe { GetClientRect(window, &mut window_rect) };

            let width = window_rect.right;
            let height = window_rect.bottom;

            // 0x00RRGGBB
            // TODO: allocate only once
            let back_buffer = vec![0x00ff0000_u32; (width * height * bytes_per_pixel) as usize];
            let bitmap_info = BITMAPINFO {
                bmiHeader: BITMAPINFOHEADER {
                    biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                    biWidth: width,
                    biHeight: -height,
                    biPlanes: 1,
                    biBitCount: 32,
                    biCompression: BI_RGB as u32,
                    biSizeImage: (width * height * bytes_per_pixel) as u32,
                    biXPelsPerMeter: 0,
                    biYPelsPerMeter: 0,
                    biClrUsed: 0,
                    biClrImportant: 0,
                },
                ..Default::default()
            };

            unsafe {
                let result = StretchDIBits(
                    device_context,
                    0,
                    0,
                    width,
                    height,
                    0,
                    0,
                    width,
                    height,
                    back_buffer.as_ptr() as *const std::ffi::c_void,
                    &bitmap_info,
                    DIB_RGB_COLORS,
                    SRCCOPY,
                );
                EndPaint(window, &paint_info);
            }

            LRESULT::default()
        }
        _ => unsafe { DefWindowProcA(window, message, wparam, lparam) },
    }
}

fn main() -> windows::Result<()> {
    let instance = unsafe { GetModuleHandleA(None) };

    let window_class = WNDCLASSEXA {
        cbSize: std::mem::size_of::<WNDCLASSEXA>() as u32,
        style: CS_HREDRAW | CS_VREDRAW | CS_OWNDC,
        lpfnWndProc: Some(wndproc),
        hInstance: instance,
        hCursor: unsafe { LoadCursorW(None, IDC_ARROW) },
        lpszClassName: PSTR(b"RustWindowClass\0".as_ptr() as _),
        ..Default::default()
    };

    unsafe {
        RegisterClassExA(&window_class);

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
