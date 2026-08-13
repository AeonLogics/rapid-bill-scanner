use super::lasersink::{
    clear_laser_channel, create_laser_channel, install_hook, is_hook_active, uninstall_hook,
};
use crate::MemoryManager;
use crate::core::extractor_state::start_worker_loop;
use std::thread::{JoinHandle, spawn};

pub struct LaserExtractor {
    worker_handle: Option<JoinHandle<()>>,
}

impl Default for LaserExtractor {
    fn default() -> Self {
        Self::new()
    }
}

impl LaserExtractor {
    pub fn new() -> Self {
        Self {
            worker_handle: None,
        }
    }

    pub fn is_active(&self) -> bool {
        is_hook_active()
    }

    pub fn start(&mut self) {
        if self.is_active() {
            return;
        }

        let receiver = create_laser_channel();
        let memory_manager = MemoryManager::init();
        install_hook();
        let handle = spawn(move || {
            start_worker_loop(receiver, memory_manager);
        });

        self.worker_handle = Some(handle);
    }

    pub fn stop(&mut self) {
        if !self.is_active() {
            return;
        }
        uninstall_hook();
        clear_laser_channel();
        if let Some(handle) = self.worker_handle.take() {
            let _ = handle.join();
        }
    }

    pub fn toggle(&mut self) -> bool {
        if self.is_active() {
            self.stop();
            false
        } else {
            self.start();
            true
        }
    }
}

impl Drop for LaserExtractor {
    fn drop(&mut self) {
        self.stop();
    }
}
