use crate::core::lasersink::LASER_KEY_TRANSMITTER;
use windows::Win32::Foundation::{LPARAM, LRESULT, WPARAM};
use windows::Win32::Media::Audio::{PlaySoundW, SND_ALIAS, SND_ASYNC};
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, KBDLLHOOKSTRUCT, WM_KEYDOWN, WM_SYSKEYDOWN,
};
use windows::core::w;
pub unsafe extern "system" fn low_level_keyboard_proc_macro(
    n_code: i32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    if n_code >= 0 {
        let msg = w_param.0 as u32;
        let is_key_down = msg == WM_KEYDOWN || msg == WM_SYSKEYDOWN;

        if is_key_down {
            let kb_struct = *(l_param.0 as *const KBDLLHOOKSTRUCT);
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

pub enum KeyAction {
    PassToOS,
    Swallow(Option<char>),
}

// THIS PART IS CREATED BY GEMINI;
pub fn process_key(vk_code: u32, flags: u32) -> KeyAction {
    // LLKHF_ALTDOWN (0x20) means Alt is physically pressed down
    let is_alt_down = (flags & 0x20) != 0;

    // 1. Never swallow Alt combos (Alt+Tab, Alt+F4, etc.)
    if is_alt_down {
        return KeyAction::PassToOS;
    }

    match vk_code {
        // Let system controls, modifiers, and function keys pass through
        0x09          // Tab
        | 0x10..=0x12 // Shift, Ctrl, Alt keys
        | 0x1B        // Escape
        | 0x5B | 0x5C // Left & Right Windows Keys
        | 0x70..=0x87 => KeyAction::PassToOS, // F1 through F24

        // Digits 0-9 (Top Row)
        0x30..=0x39 => {
            let ch = (b'0' + (vk_code - 0x30) as u8) as char;
            KeyAction::Swallow(Some(ch))
        }

        // Digits 0-9 (Numpad)
        0x60..=0x69 => {
            let ch = (b'0' + (vk_code - 0x60) as u8) as char;
            KeyAction::Swallow(Some(ch))
        }

        // Letters A-Z
        0x41..=0x5A => {
            let ch = (b'A' + (vk_code - 0x41) as u8) as char;
            KeyAction::Swallow(Some(ch))
        }

        // Barcode separators & terminators
        0x0D => KeyAction::Swallow(Some('\n')), // Enter / Return
        0x20 => KeyAction::Swallow(Some(' ')),  // Space
        0xBD => KeyAction::Swallow(Some('-')),  // Dash / Minus
        0xBE => KeyAction::Swallow(Some('.')),  // Period
        0xBF => KeyAction::Swallow(Some('/')),  // Forward Slash

        // Unknown keys while scanner is active -> swallow without sending char
        _ => KeyAction::Swallow(None),
    }
}

// free from AI...
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
