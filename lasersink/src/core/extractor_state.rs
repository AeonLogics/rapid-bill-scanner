use crate::core::lasersink::LaserReceiver;
use crate::core::memory_manager::MemoryManager;
use std::sync::mpsc::{Receiver, RecvTimeoutError};
use std::time::Duration;

#[derive(Default)]
pub struct ExtractorState {
    pub collected_data: String,
}

impl ExtractorState {
    pub fn clear(&mut self) {
        self.collected_data.clear();
    }

    pub fn push(&mut self, ch: char) {
        self.collected_data.push(ch);
    }

    pub fn is_empty(&self) -> bool {
        self.collected_data.is_empty()
    }
}

pub fn start_worker_loop(receiver: LaserReceiver) {
    let mut state = ExtractorState::default();
    let memory_manager = MemoryManager {};

    loop {
        match receiver.recv_timeout(Duration::from_millis(30)) {
            Ok(ch) => {
                if ch == '\n' || ch == '\r' {
                    if !state.is_empty() {
                        let barcode = state.collected_data.clone();

                        // --------------------------------------------------
                        // PIPELINE NEXT STEPS:
                        // 1. Pass `barcode` to BillExtractor trait logic
                        // 2. Feed processed result to key simulator
                        // --------------------------------------------------

                        state.clear();
                    }
                } else {
                    state.push(ch);
                }
            }

            Err(RecvTimeoutError::Timeout) => {
                if !state.is_empty() {
                    state.clear();
                }
            }

            Err(RecvTimeoutError::Disconnected) => {
                println!("Laser worker shutting down.");
                break;
            }
        }
    }
}
