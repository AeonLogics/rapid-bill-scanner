use windows::Win32::Foundation::{LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::KBDLLHOOKSTRUCT;

pub unsafe extern "system" fn low_level_keyboard_proc_macro(
    n_code: i32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    print_key(l_param);
    LRESULT(1)
}

pub fn print_key(l_param: LPARAM) {
    unsafe {
        let kb_struct = *(l_param.0 as *const KBDLLHOOKSTRUCT);
        let vk_code = kb_struct.vkCode;
        let scan_code = kb_struct.scanCode;
        let flags = kb_struct.flags;

        println!("Virtual Key: {}, Scan Code: {}", vk_code, scan_code);
    }
}
