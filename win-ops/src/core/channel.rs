use crate::core::Executable;
use crate::core::window::{IS_PASSTHROUGH, low_level_keyboard_proc_macro};
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

/// Runs a block of key actions while disabling the hook's swallowing behavior.
fn send_passthrough<F: FnOnce()>(action: F) {
    IS_PASSTHROUGH.store(true, Ordering::Relaxed);
    action();
    IS_PASSTHROUGH.store(false, Ordering::Relaxed);
}

pub struct LaserChannel {
    is_running: Arc<AtomicBool>,
    worker_handle: Mutex<Option<JoinHandle<()>>>,
    last_bill: Arc<Mutex<Option<LaserBill>>>,
}

impl Global for LaserChannel {}

impl LaserChannel {
    pub fn init(cx: &mut App) {
        cx.set_global(Self {
            is_running: Arc::new(AtomicBool::new(false)),
            worker_handle: Mutex::new(None),
            last_bill: Arc::new(Mutex::new(None)),
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

    pub fn toggle(&mut self, target_software: TargetSoftware) {
        if self.status() {
            self.stop();
            return;
        }

        if let Some(rx) = Self::attach_hook() {
            self.is_running.store(true, Ordering::Relaxed);
            let running_flag = Arc::clone(&self.is_running);
            let last_bill_store = Arc::clone(&self.last_bill);

            let handle = thread::spawn(move || {
                let mut collector = StrokeCollector::default();

                while running_flag.load(Ordering::Relaxed) {
                    match rx.recv_timeout(Duration::from_millis(80)) {
                        Ok('\n' | '\r') => {
                            if !collector.is_empty() {
                                let raw_barcode = std::mem::take(&mut collector.collected_data);

                                if let Some(bill) = LaserBill::parse(raw_barcode.clone()) {
                                    // Save scanned payload safely across thread
                                    if let Ok(mut lock) = last_bill_store.lock() {
                                        *lock = Some(bill.clone());
                                    }

                                    send_passthrough(|| {
                                        target_software
                                            .build_execute(&bill.reference, "03007277148")
                                            .into_iter()
                                            .for_each(|e| e.execute_self());
                                    });
                                } else {
                                    play_notification();
                                    send_passthrough(|| {
                                        KeyAction::Text(raw_barcode).execute_self();
                                        KeyAction::Enter.execute_self();
                                    });
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
                                send_passthrough(|| {
                                    KeyAction::Text(typed_data).execute_self();
                                });
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
            if thread::current().id() != handle.thread().id() {
                let _ = handle.join();
            }
        }
    }

    pub fn status(&self) -> bool {
        is_hook_active()
    }

    pub fn last_bill(&self) -> Option<LaserBill> {
        self.last_bill.lock().ok()?.clone()
    }
}

impl Drop for LaserChannel {
    fn drop(&mut self) {
        self.stop();
    }
}
