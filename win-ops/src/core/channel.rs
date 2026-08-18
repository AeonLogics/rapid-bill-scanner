use crate::core::Executable;
use crate::core::window::low_level_keyboard_proc_macro;
use crate::{StrokeCollector, play_notification};
use gpui::{App, Global};
use primitives::{KeyAction, LaserBill, TargetSoftware};
use std::sync::atomic::{AtomicBool, AtomicIsize, Ordering};
use std::sync::mpsc::{Receiver, RecvTimeoutError, Sender, channel};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;
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

pub struct LaserChannel {
    is_running: Arc<AtomicBool>,
    worker_handle: Mutex<Option<JoinHandle<()>>>,
}

impl Global for LaserChannel {}

impl LaserChannel {
    pub fn init(cx: &mut App) {
        cx.set_global(Self {
            is_running: Arc::new(AtomicBool::new(false)),
            worker_handle: Mutex::new(None),
        });
    }

    fn attach_hook() -> Option<LaserReceiver> {
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

    fn detach_hook() {
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

    /// Associated function—no `&self` needed! Safe to invoke inside thread::spawn closures.
    fn execute_with_hook_passthrough<F>(
        running_flag: &AtomicBool,
        action: F,
    ) -> Option<LaserReceiver>
    where
        F: FnOnce(),
    {
        Self::detach_hook();

        action();

        if running_flag.load(Ordering::Relaxed) {
            return Self::attach_hook();
        }
        None
    }

    pub fn toggle(&self, target_software: TargetSoftware) {
        if self.status() {
            self.stop();
            return;
        }

        if let Some(mut rx) = Self::attach_hook() {
            self.is_running.store(true, Ordering::Relaxed);
            let running_flag = Arc::clone(&self.is_running);

            let handle = thread::spawn(move || {
                let mut collector = StrokeCollector::default();

                while running_flag.load(Ordering::Relaxed) {
                    // Bumping to 80ms prevents timeout splits during fast typing or slow HID scanners
                    match rx.recv_timeout(Duration::from_millis(80)) {
                        Ok('\n' | '\r') => {
                            if !collector.is_empty() {
                                let raw_barcode = std::mem::take(&mut collector.collected_data);

                                if let Some(bill) = LaserBill::parse(raw_barcode.clone()) {
                                    if let Some(new_rx) =
                                        Self::execute_with_hook_passthrough(&running_flag, || {
                                            bill.execute(target_software, "03000000000");
                                        })
                                    {
                                        rx = new_rx;
                                    }
                                } else {
                                    play_notification();
                                    if let Some(new_rx) =
                                        Self::execute_with_hook_passthrough(&running_flag, || {
                                            KeyAction::Text(raw_barcode).execute_self();
                                            KeyAction::Enter.execute_self();
                                        })
                                    {
                                        rx = new_rx;
                                    }
                                }
                            }
                            collector.clear();
                        }
                        Ok(ch) => {
                            collector.push(ch);
                        }
                        Err(RecvTimeoutError::Timeout) => {
                            if !collector.is_empty() {
                                let typed_data = std::mem::take(&mut collector.collected_data);
                                if let Some(new_rx) =
                                    Self::execute_with_hook_passthrough(&running_flag, || {
                                        KeyAction::Text(typed_data).execute_self();
                                    })
                                {
                                    rx = new_rx;
                                }
                            }
                            collector.clear();
                        }
                        Err(RecvTimeoutError::Disconnected) => break,
                    }
                }

                Self::detach_hook();
            });

            *self.worker_handle.lock().unwrap() = Some(handle);
        }
    }

    pub fn stop(&self) {
        self.is_running.store(false, Ordering::Relaxed);
        Self::detach_hook();

        if let Some(handle) = self.worker_handle.lock().unwrap().take() {
            // Guard against self-joining if called within thread context
            if thread::current().id() != handle.thread().id() {
                let _ = handle.join();
            }
        }
    }

    pub fn status(&self) -> bool {
        is_hook_active()
    }
}

impl Drop for LaserChannel {
    fn drop(&mut self) {
        self.stop();
    }
}
