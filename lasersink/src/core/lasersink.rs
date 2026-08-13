use crate::core::window::low_level_keyboard_proc_macro;
use std::sync::Mutex;
use std::sync::atomic::{AtomicIsize, Ordering};
use std::sync::mpsc::{Receiver, Sender, channel};
use windows::Win32::Foundation::HINSTANCE;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::{
    HHOOK, SetWindowsHookExW, UnhookWindowsHookEx, WH_KEYBOARD_LL,
};

pub type LaserTransmitter = Sender<char>;
pub type LaserReceiver = Receiver<char>;

pub(super) static H_HOOK: AtomicIsize = AtomicIsize::new(0);
pub(super) static LASER_KEY_TRANSMITTER: Mutex<Option<LaserTransmitter>> = Mutex::new(None);

pub fn is_hook_active() -> bool {
    H_HOOK.load(Ordering::Relaxed) != 0
}

pub fn install_hook() {
    if is_hook_active() {
        return;
    }

    unsafe {
        let instance = GetModuleHandleW(None).expect("Failed to get module handle");
        let hook = SetWindowsHookExW(
            WH_KEYBOARD_LL,
            Some(low_level_keyboard_proc_macro),
            Some(HINSTANCE(instance.0)),
            0,
        )
        .expect("failed to SetWindowsHookExW");

        H_HOOK.store(hook.0 as usize as isize, Ordering::Relaxed);
    }
}

pub fn uninstall_hook() {
    let raw = H_HOOK.swap(0, Ordering::Relaxed);

    if raw != 0 {
        unsafe {
            let raw_ptr = raw as usize as *mut core::ffi::c_void;
            let handle = HHOOK(raw_ptr);
            let _ = UnhookWindowsHookEx(handle);
        }
    }
}

pub fn create_laser_channel() -> LaserReceiver {
    let (tx, rx) = channel();
    LASER_KEY_TRANSMITTER.lock().unwrap().replace(tx);
    rx
}

pub fn clear_laser_channel() {
    LASER_KEY_TRANSMITTER.lock().unwrap().take();
}
