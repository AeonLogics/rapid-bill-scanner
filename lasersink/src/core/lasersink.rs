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

// --- Private Internals ---
static H_HOOK: AtomicIsize = AtomicIsize::new(0);
static LASER_KEY_TRANSMITTER: Mutex<Option<LaserTransmitter>> = Mutex::new(None);

fn is_hook_active() -> bool {
    H_HOOK.load(Ordering::Relaxed) != 0
}

// --- Public Library Interface ---
pub struct LaserChannel {
    is_open: bool,
}

impl LaserChannel {
    /// Creates a dormant handle. No hooks attached yet.
    pub fn init() -> Self {
        Self { is_open: false }
    }

    /// Installs the low-level OS hook and returns the receiver.
    pub fn open(&mut self) -> Option<LaserReceiver> {
        if self.is_open || is_hook_active() {
            return None; // Already running
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

        self.is_open = true;
        Some(rx)
    }

    /// Explicitly closes the channel and unhooks the OS listener.
    pub fn close(&mut self) {
        if !self.is_open {
            return;
        }

        // Clear transmitter so hook callback stops sending
        *LASER_KEY_TRANSMITTER.lock().unwrap() = None;

        // Unhook from Win32
        let raw = H_HOOK.swap(0, Ordering::Relaxed);
        if raw != 0 {
            unsafe {
                let raw_ptr = raw as usize as *mut core::ffi::c_void;
                let handle = HHOOK(raw_ptr);
                let _ = UnhookWindowsHookEx(handle);
            }
        }

        self.is_open = false;
    }

    pub fn is_active(&self) -> bool {
        self.is_open && is_hook_active()
    }
}

// Automatic cleanup! If LaserChannel drops, Windows unhooks safely.
impl Drop for LaserChannel {
    fn drop(&mut self) {
        self.close();
    }
}
