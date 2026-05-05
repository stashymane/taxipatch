use windows::core::HSTRING;
use windows::Win32::System::Diagnostics::Debug::OutputDebugStringW;
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_ICONINFORMATION, MB_OK};

pub fn message_box<S1: AsRef<str>, S2: AsRef<str>>(title: S1, text: S2)
where
    HSTRING: From<S1>,
    HSTRING: From<S2>,
{
    unsafe {
        let _ = MessageBoxW(
            None,
            &HSTRING::from(text),
            &HSTRING::from(title),
            MB_OK | MB_ICONINFORMATION,
        );
    }
}

pub fn debug_log<S: AsRef<str>>(message: S)
where
    HSTRING: From<S>,
{
    unsafe { OutputDebugStringW(&HSTRING::from(message)) }
}
