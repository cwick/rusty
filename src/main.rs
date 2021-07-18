use bindings::Windows::Win32::UI::WindowsAndMessaging::*;

fn main() -> windows::Result<()> {
    unsafe {
        MessageBoxA(None, "Text", "Caption", MB_OK);
    }

    Ok(())
}
