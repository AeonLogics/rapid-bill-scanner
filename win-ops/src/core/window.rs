use crate::core::channel::LASER_KEY_TRANSMITTER;
use std::sync::atomic::{AtomicBool, Ordering};
use windows::Win32::Foundation::{LPARAM, LRESULT, WPARAM};
use windows::Win32::Media::Audio::{PlaySoundW, SND_ALIAS, SND_ASYNC};
use windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, KBDLLHOOKSTRUCT, LLKHF_INJECTED, WM_KEYDOWN, WM_SYSKEYDOWN,
};
use windows::core::w;

// Global flag to temporarily bypass swallowing during programmatic typing
pub static IS_PASSTHROUGH: AtomicBool = AtomicBool::new(false);

pub unsafe extern "system" fn low_level_keyboard_proc_macro(
    n_code: i32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    if n_code >= 0 {
        let kb_struct = *(l_param.0 as *const KBDLLHOOKSTRUCT);

        // 1. Automatically ignore keys injected by SendInput (KeyAction)
        let is_injected = (kb_struct.flags.0 & LLKHF_INJECTED.0) != 0;
        // 2. Ignore keys if passthrough mode is actively turned on
        let is_bypassed = IS_PASSTHROUGH.load(Ordering::Relaxed);

        if is_injected || is_bypassed {
            return CallNextHookEx(None, n_code, w_param, l_param);
        }

        let msg = w_param.0 as u32;
        let is_key_down = msg == WM_KEYDOWN || msg == WM_SYSKEYDOWN;

        if is_key_down {
            let vk_code = kb_struct.vkCode;
            let flags = kb_struct.flags.0;

            if let KeyAction::Swallow(maybe_char) = process_key(vk_code, flags) {
                if let Ok(guard) = LASER_KEY_TRANSMITTER.lock()
                    && let Some(tx) = guard.as_ref()
                {
                    if let Some(ch) = maybe_char {
                        let _ = tx.send(ch);
                    }
                    return LRESULT(1);
                }
            }
        }
    }

    CallNextHookEx(None, n_code, w_param, l_param)
}

enum KeyAction {
    PassToOS,
    Swallow(Option<char>),
}
fn process_key(vk_code: u32, flags: u32) -> KeyAction {
    let is_alt_down = (flags & 0x20) != 0;
    let is_ctrl_down = unsafe { (GetAsyncKeyState(0x11) as u16 & 0x8000) != 0 };

    // Pass Alt or Ctrl shortcuts straight through to Windows
    if is_alt_down || is_ctrl_down {
        return KeyAction::PassToOS;
    }

    match vk_code {
        // 0x08 (Backspace) MUST pass through so Windows can delete text on screen!
        0x08 | 0x09 | 0x10..=0x12 | 0x1B | 0x5B | 0x5C | 0x70..=0x87 => KeyAction::PassToOS,

        // Digits 0-9 (Top Row)
        0x30..=0x39 => KeyAction::Swallow(Some((b'0' + (vk_code - 0x30) as u8) as char)),
        // Digits 0-9 (Numpad)
        0x60..=0x69 => KeyAction::Swallow(Some((b'0' + (vk_code - 0x60) as u8) as char)),
        // Letters A-Z
        0x41..=0x5A => KeyAction::Swallow(Some((b'A' + (vk_code - 0x41) as u8) as char)),

        // Separators & terminators
        0x0D => KeyAction::Swallow(Some('\n')),
        0x20 => KeyAction::Swallow(Some(' ')),
        0xBD => KeyAction::Swallow(Some('-')),
        0xBE => KeyAction::Swallow(Some('.')),
        0xBF => KeyAction::Swallow(Some('/')),

        _ => KeyAction::Swallow(None),
    }
}

pub fn play_notification() {
    unsafe {
        let _ = PlaySoundW(w!("SystemAsterisk"), None, SND_ALIAS | SND_ASYNC);
    }
}

pub fn play_bonk_error() {
    unsafe {
        let _ = PlaySoundW(w!("SystemHand"), None, SND_ALIAS | SND_ASYNC);
    }
}
