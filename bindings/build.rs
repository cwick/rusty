fn main() {
  windows::build! {
      Windows::Win32::Foundation::{HINSTANCE, PSTR, RECT},
      Windows::Win32::System::LibraryLoader::GetModuleHandleA,
      Windows::Win32::System::WindowsProgramming::INFINITE,
      Windows::Win32::UI::WindowsAndMessaging::{
          MessageBoxA,
          AdjustWindowRect, CreateWindowExA, DefWindowProcA, DispatchMessageA, GetWindowLongA,
          GetWindowLongPtrA, LoadCursorW, PeekMessageA, PostQuitMessage, RegisterClassExA,
          SetWindowLongA, SetWindowLongPtrA, ShowWindow, TranslateMessage, CREATESTRUCTA,
          CW_USEDEFAULT, IDC_ARROW, MSG, WM_CREATE, WM_DESTROY, WM_KEYDOWN, WM_KEYUP, WM_PAINT,
          WM_QUIT, WNDCLASSEXA, WNDCLASS_STYLES,
      },
  };
}
