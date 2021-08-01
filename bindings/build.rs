fn main() {
    windows::build! {
        Windows::Win32::Graphics::Gdi::{BeginPaint, CreateCompatibleDC,
            EndPaint, PatBlt, StretchDIBits, ValidateRect,
            DeleteDC,  BI_RGB},
        Windows::Win32::Foundation::{HINSTANCE, PSTR, RECT, HWND, WPARAM, LPARAM, LRESULT},
        Windows::Win32::System::LibraryLoader::GetModuleHandleA,
        Windows::Win32::System::Performance::{QueryPerformanceCounter,QueryPerformanceFrequency},
        Windows::Win32::System::WindowsProgramming::INFINITE,
        Windows::Win32::UI::WindowsAndMessaging::{
            MessageBoxA, AdjustWindowRectEx,
            AdjustWindowRect, CreateWindowExA, DefWindowProcA, DispatchMessageA,
            GetWindowLongPtrA, LoadCursorW, PeekMessageA, GetMessageA, PostQuitMessage, RegisterClassExA,
            SetWindowLongA, ShowWindow, TranslateMessage,
            GetClientRect, SetWindowLongPtrA, CREATESTRUCTA,
            CW_USEDEFAULT, IDC_ARROW, MSG, WM_CREATE, WM_DESTROY, WM_KEYDOWN, WM_KEYUP, WM_PAINT,
            WM_SIZE, WM_QUIT, WNDCLASSEXA, WNDCLASS_STYLES
        },
    };
}
