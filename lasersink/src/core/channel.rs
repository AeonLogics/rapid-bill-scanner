use crate::core::window::low_level_keyboard_proc_macro;
use std::sync::Mutex;
use std::sync::atomic::{AtomicIsize, Ordering};
use std::sync::mpsc::{Receiver, Sender, channel};
use windows::Win32::Foundation::HINSTANCE;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::{
    HHOOK, SetWindowsHookExW, UnhookWindowsHookEx, WH_KEYBOARD_LL,
};

pub type LaserReceiver = Receiver<char>;
type LaserTransmitter = Sender<char>;

static H_HOOK: AtomicIsize = AtomicIsize::new(0);
pub static LASER_KEY_TRANSMITTER: Mutex<Option<LaserTransmitter>> = Mutex::new(None);

fn is_hook_active() -> bool {
    H_HOOK.load(Ordering::Relaxed) != 0
}

pub struct LaserChannel;

impl LaserChannel {
    pub fn init() -> Self {
        Self
    }

    pub fn attach(&mut self) -> Option<LaserReceiver> {
        if is_hook_active() {
            return None;
        }

        unsafe {
            let instance = GetModuleHandleW(None).ok()?;
            let hook = SetWindowsHookExW(
                WH_KEYBOARD_LL,
                Some(low_level_keyboard_proc_macro),
                Some(HINSTANCE(instance.0)),
                0,
            )
            .ok()?;

            H_HOOK.store(hook.0 as usize as isize, Ordering::Relaxed);
        }

        let (tx, rx) = channel();
        *LASER_KEY_TRANSMITTER.lock().unwrap() = Some(tx);

        Some(rx)
    }

    pub fn detach(&mut self) {
        if !is_hook_active() {
            return;
        }

        *LASER_KEY_TRANSMITTER.lock().unwrap() = None;

        let raw = H_HOOK.swap(0, Ordering::Relaxed);
        if raw != 0 {
            unsafe {
                let raw_ptr = raw as usize as *mut core::ffi::c_void;
                let handle = HHOOK(raw_ptr);
                let _ = UnhookWindowsHookEx(handle);
            }
        }
    }

    pub fn status(&self) -> bool {
        is_hook_active()
    }
}

impl Drop for LaserChannel {
    fn drop(&mut self) {
        self.detach();
    }
}
