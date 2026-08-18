use crate::core::channel::LaserReceiver;
use std::sync::mpsc::{Receiver, RecvTimeoutError};
use std::time::Duration;

#[derive(Default)]
pub struct StrokeCollector {
    pub collected_data: String,
}

impl StrokeCollector {
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
