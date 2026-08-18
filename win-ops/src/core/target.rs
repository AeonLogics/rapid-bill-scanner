use primitives::KeyAction;
use std::mem::size_of;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, KEYEVENTF_UNICODE, SendInput,
    VIRTUAL_KEY, VK_RETURN, VK_SPACE, VK_TAB,
};

pub trait Executable {
    fn execute_self(self);
}
impl Executable for KeyAction {
    fn execute_self(self) {
        match self {
            KeyAction::Text(d) => send_ch(d),
            KeyAction::Tab => send_vk(VK_TAB),
            KeyAction::Space => send_vk(VK_SPACE),
            KeyAction::Enter => send_vk(VK_RETURN),
        }
    }
}

fn send_vk(vk: VIRTUAL_KEY) {
    let inputs = [
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: vk,
                    ..Default::default()
                },
            },
        },
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: vk,
                    dwFlags: KEYEVENTF_KEYUP,
                    ..Default::default()
                },
            },
        },
    ];

    unsafe {
        SendInput(&inputs, size_of::<INPUT>() as i32);
    }
}

fn send_ch(text: String) {
    for ch in text.encode_utf16() {
        let input = [
            INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VIRTUAL_KEY(0),
                        wScan: ch,
                        dwFlags: KEYEVENTF_UNICODE, // 2. Explicitly tell Windows this is a unicode char
                        ..Default::default()
                    },
                },
            },
            INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VIRTUAL_KEY(0),
                        wScan: ch,
                        dwFlags: KEYEVENTF_UNICODE | KEYEVENTF_KEYUP,
                        ..Default::default()
                    },
                },
            },
        ];

        unsafe {
            SendInput(&input, size_of::<INPUT>() as i32);
        }
    }
}
