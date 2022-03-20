use windows::{core::*, Win32::Foundation::*, Win32::UI::WindowsAndMessaging::*};

// バックグラウンドからキーボード入力を取得する
// https://qiita.com/horyu/items/12f6cd13ceb217782df3
// の、「4. バックグラウンドからキーボード入力を取得して表示する」

pub fn kii_risunaa2() -> Result<()> {
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

static mut INPUTS_ARRAY: [bool; 256] = [false; 256];
unsafe fn set_and_show(vk_code: &u16, tf: bool) {
    INPUTS_ARRAY[*vk_code as usize] = tf;
    let mut s = String::with_capacity((b'Z' - b'A' + 1) as usize);
    for i in (b'A' as usize)..=(b'Z' as usize) {
        s.push(if INPUTS_ARRAY[i] { 'T' } else { 'F' });
    }
    println!("{s}");
}

extern "system" fn k_callback1(ncode: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        if ncode as u32 == HC_ACTION {
            match wparam.0 as u32 {
                WM_KEYDOWN => set_and_show(&*(lparam.0 as *const u16) as &u16, true),
                WM_KEYUP => set_and_show(&*(lparam.0 as *const u16) as &u16, false),
                _ => (),
            }
        }
        CallNextHookEx(HHOOK::default(), ncode, wparam, lparam)
    }
}
