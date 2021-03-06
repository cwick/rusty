use bindings::Windows::Win32::Foundation::*;
use bindings::Windows::Win32::Graphics::Gdi::*;
use bindings::Windows::Win32::System::LibraryLoader::*;
use bindings::Windows::Win32::System::Performance::{
    QueryPerformanceCounter, QueryPerformanceFrequency,
};
use bindings::Windows::Win32::UI::WindowsAndMessaging::*;

type Win32BitmapData = Vec<u32>;
struct Win32Bitmap {
    data: Win32BitmapData,
    width: usize,
    height: usize,
    bytes_per_pixel: u8,
    bitmap_info: BITMAPINFO,
}

macro_rules! benchmark {
    ($context:literal, $($e:tt)+) => {
        let mut start: i64 = 0;
        let mut frequency: i64 = 0;
        #[allow(unused_unsafe)]
        unsafe{
            QueryPerformanceCounter(&mut start);
            // TODO: cache this
            QueryPerformanceFrequency(&mut frequency);
        }
        $($e)*

        let mut end: i64 = 0;
        #[allow(unused_unsafe)]
        unsafe{ QueryPerformanceCounter(&mut end); }

        // TODO: This takes 0.5 ms. Draw to screen instead?
        println!(
            "{}: {} ms",
            $context,
            (1000 * (end - start)) as f64 / frequency as f64
        );
    };
}

impl Win32Bitmap {
    pub fn new() -> Win32Bitmap {
        let width = 800_usize;
        let height = 600_usize;
        let bytes_per_pixel = 4_u8;

        Win32Bitmap {
            data: vec![0x00000000_u32; width * height],
            width,
            height,
            bytes_per_pixel,
            bitmap_info: Win32Bitmap::create_bitmap_info(width, height, bytes_per_pixel),
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, value: u32) {
        self.data[y * self.width + x] = value;
    }

    fn create_bitmap_info(width: usize, height: usize, bytes_per_pixel: u8) -> BITMAPINFO {
        BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: width as i32,
                biHeight: -(height as i32),
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB as u32,
                biSizeImage: (width * height * bytes_per_pixel as usize) as u32,
                biXPelsPerMeter: 0,
                biYPelsPerMeter: 0,
                biClrUsed: 0,
                biClrImportant: 0,
            },
            ..Default::default()
        }
    }
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match message {
        // WM_CLOSE => {
        // Calls DestroyWindow by default
        // }
        WM_CREATE => {
            let create_info = unsafe { &*(lparam.0 as *const CREATESTRUCTA) };
            let bitmap = create_info.lpCreateParams as *const Win32Bitmap;
            unsafe {
                SetWindowLongPtrA(window, GWLP_USERDATA, bitmap as isize);
            }

            LRESULT::default()
        }

        // Called after window is destroyed
        WM_DESTROY => {
            // Posts WM_QUIT to the message queue
            unsafe { PostQuitMessage(0) };
            LRESULT::default()
        }

        // WM_SIZE => LRESULT::default(),
        WM_PAINT => {
            // TODO: Measure how long a paint takes.
            let bitmap =
                unsafe { &mut *(GetWindowLongPtrA(window, GWLP_USERDATA) as *mut Win32Bitmap) };

            let mut paint_info: PAINTSTRUCT = Default::default();
            let device_context = unsafe { BeginPaint(window, &mut paint_info) };
            let mut window_rect = RECT::default();
            unsafe { GetClientRect(window, &mut window_rect) };

            let client_width = window_rect.right;
            let client_height = window_rect.bottom;

            // Upper left
            const RED: u32 = 0x00ff0000;
            const GREEN: u32 = 0x0000ff00;
            const BLUE: u32 = 0x000000ff;

            for x in 0..10 {
                bitmap.set_pixel(x, 0, RED);
            }
            for y in 0..10 {
                bitmap.set_pixel(0, y, RED);
            }
            bitmap.set_pixel(0, 0, GREEN);

            // Lower left
            for x in 0..10 {
                bitmap.set_pixel(x, bitmap.height - 1, RED);
            }
            for y in 0..10 {
                bitmap.set_pixel(0, bitmap.height - y - 1, RED);
            }
            bitmap.set_pixel(0, bitmap.height - 1, GREEN);

            // Lower right
            for x in 0..10 {
                bitmap.set_pixel(bitmap.width - x - 1, bitmap.height - 1, RED);
            }
            for y in 0..10 {
                bitmap.set_pixel(bitmap.width - 1, bitmap.height - y - 1, RED);
            }
            bitmap.set_pixel(bitmap.width - 1, bitmap.height - 1, GREEN);

            // Upper right
            for x in 0..10 {
                bitmap.set_pixel(bitmap.width - 1 - x, 0, RED);
            }
            for y in 0..10 {
                bitmap.set_pixel(bitmap.width - 1, y, RED);
            }
            bitmap.set_pixel(bitmap.width - 1, 0, GREEN);

            unsafe {
                benchmark! {"StretchDIBits",
                    StretchDIBits(
                        device_context,
                        0,
                        0,
                        client_width,
                        client_height,
                        0,
                        0,
                        bitmap.width as i32,
                        bitmap.height as i32,
                        bitmap.data.as_ptr() as *const std::ffi::c_void,
                        &bitmap.bitmap_info,
                        DIB_RGB_COLORS,
                        SRCCOPY,
                    );
                };
                EndPaint(window, &paint_info);
            }

            LRESULT::default()
        }
        _ => unsafe { DefWindowProcA(window, message, wparam, lparam) },
    }
}

fn main() -> windows::Result<()> {
    let instance = unsafe { GetModuleHandleA(None) };
    let mut bitmap = Win32Bitmap::new();

    register_window_class(instance);
    create_and_show_window(instance, &mut bitmap);

    loop {
        let mut message = MSG::default();

        if unsafe { GetMessageA(&mut message, None, 0, 0) }.into() {
            benchmark! {"Main loop",
                unsafe {
                    TranslateMessage(&message);
                    DispatchMessageA(&message);
                }
            }
        } else {
            break;
        }
    }

    Ok(())
}

fn register_window_class(instance: HINSTANCE) {
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
    }
}

fn create_and_show_window(instance: HINSTANCE, bitmap: &mut Win32Bitmap) -> HWND {
    let window_style = WS_OVERLAPPEDWINDOW;
    let (window_width, window_height) =
        calculate_window_size(window_style, bitmap.width as i32, bitmap.height as i32);

    unsafe {
        let hwnd = CreateWindowExA(
            Default::default(),
            "RustWindowClass",
            "Hello Windows",
            window_style,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            window_width,
            window_height,
            None, // no parent window
            None, // no menus
            instance,
            bitmap as *mut _ as _,
        );

        ShowWindow(hwnd, SW_SHOW);
        hwnd
    }
}

fn calculate_window_size(style: WINDOW_STYLE, client_width: i32, client_height: i32) -> (i32, i32) {
    let mut rect = RECT {
        left: 0,
        right: client_width,
        top: 0,
        bottom: client_height,
    };

    unsafe {
        AdjustWindowRectEx(&mut rect, style, false, Default::default());
    }

    let window_width = rect.right - rect.left;
    let window_height = rect.bottom - rect.top;
    (window_width, window_height)
}
