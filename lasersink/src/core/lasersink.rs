use crate::core::window::low_level_keyboard_proc_macro;
use std::sync::Mutex;
use std::sync::atomic::{AtomicIsize, Ordering};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel};
use windows::Win32::Foundation::HINSTANCE;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::{
    HHOOK, SetWindowsHookExW, UnhookWindowsHookEx, WH_KEYBOARD_LL,
};

type LaserTransmitter = UnboundedSender<i32>;
pub type LaserReceiver = UnboundedReceiver<i32>;

pub(crate) static H_HOOK: AtomicIsize = AtomicIsize::new(0);
pub static LASER_KEY_TRANSMITTER: Mutex<Option<LaserTransmitter>> = Mutex::new(None);

pub struct LaserSink {
    pub receiver: Option<LaserReceiver>,
}

impl LaserSink {
    pub fn init() -> Self {
        let (tx, rx) = unbounded_channel();
        LASER_KEY_TRANSMITTER.lock().unwrap().replace(tx);
        Self { receiver: Some(rx) }
    }

    /// Takes ownership of the receiver once for the extractor wrapper
    pub(crate) fn take_receiver(&mut self) -> Option<LaserReceiver> {
        self.receiver.take()
    }

    pub fn is_active(&self) -> bool {
        H_HOOK.load(Ordering::Relaxed) != 0
    }

    pub fn register_proc_macro_hook(&self) {
        if self.is_active() {
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

    pub fn unregister_proc_macro_hook(&self) {
        let raw = H_HOOK.swap(0, Ordering::Relaxed);

        if raw != 0 {
            unsafe {
                let raw_ptr = raw as usize as *mut core::ffi::c_void;
                let handle = HHOOK(raw_ptr);
                let _ = UnhookWindowsHookEx(handle);
            }
        }
    }

    pub fn toggle_active(&self) -> bool {
        if self.is_active() {
            self.unregister_proc_macro_hook();
            false
        } else {
            self.register_proc_macro_hook();
            true
        }
    }
}

impl Drop for LaserSink {
    fn drop(&mut self) {
        self.unregister_proc_macro_hook();
    }
}
