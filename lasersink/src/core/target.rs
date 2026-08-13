use crate::bills::LaserBill;
use crate::core::target::KeyAction::Text;
use windows::Win32::Media::Audio::In;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, SendInput, VIRTUAL_KEY, VK_RETURN,
    VK_SPACE, VK_TAB,
};

pub enum KeyAction {
    Text(String),
    Tab,
    Enter,
    Space,
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub enum TargetSoftware {
    #[default]
    Nadra,
}

impl TargetSoftware {
    pub const fn init() -> TargetSoftware {
        TargetSoftware::Nadra
    }
    fn label(&self) -> &'static str {
        match self {
            TargetSoftware::Nadra => "Nadra",
        }
    }

    fn build_key_press_sequence(&self, bill: LaserBill, contact: String) -> Vec<KeyAction> {
        match self {
            TargetSoftware::Nadra => {
                vec![
                    KeyAction::Text(bill.reference_number),
                    KeyAction::Tab,
                    KeyAction::Tab,
                    KeyAction::Tab,
                    KeyAction::Text(contact),
                    KeyAction::Tab,
                    KeyAction::Space,
                    KeyAction::Tab,
                    KeyAction::Tab,
                    KeyAction::Space,
                ]
            }
        }
    }
}

pub fn execute_sequence(actions: Vec<KeyAction>) {
    for action in actions {
        match action {
            Text(d) => {}
            KeyAction::Tab => send_vk(VK_TAB),
            KeyAction::Enter => send_vk(VK_RETURN),
            KeyAction::Space => send_vk(VK_SPACE),
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
                        dwFlags: KEYEVENTF_KEYUP,
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
