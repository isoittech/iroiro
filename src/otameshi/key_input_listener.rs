use windows::{core::*, Win32::Foundation::*, Win32::UI::WindowsAndMessaging::*};

pub fn kii_risunaa() -> Result<()> {
    unsafe {
        let k_hook = SetWindowsHookExA(WH_KEYBOARD_LL, Some(k_callback1), HINSTANCE::default(), 0);
        let mut message = MSG::default();
        while GetMessageA(&mut message, HWND::default(), 0, 0).into() {
            DispatchMessageA(&message);
        }
        if !k_hook.is_invalid() {
            UnhookWindowsHookEx(k_hook);
        }
        Ok(())
    }
}

extern "system" fn k_callback1(ncode: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        if wparam.0 as u32 == WM_KEYDOWN && ncode as u32 == HC_ACTION {
            let vk_code_inner = &*(lparam.0 as *const u16) as &u16;
            dbg!(vk_code_inner);
        }
        CallNextHookEx(HHOOK::default(), ncode, wparam, lparam)
    }
}
