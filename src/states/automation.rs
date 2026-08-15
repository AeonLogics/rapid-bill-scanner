use crate::states::BillManager;
use crate::views::AutomationView;
use gpui::{AsyncApp, Context};
use lasersink::StrokeCollector;
use std::sync::mpsc::{Receiver, RecvTimeoutError};
use std::time::Duration;

impl AutomationView {
    pub fn toggle_automation(&mut self, cx: &mut Context<Self>) {
        if self.transmit_channel.status() {
            self.transmit_channel.detach();
        } else if let Some(rx) = self.transmit_channel.attach() {
            start_barcode_listener(rx, cx);
        }

        cx.notify();
    }
}

/// Spawns the background listener thread and bridges scans back to GPUI.
fn start_barcode_listener(rx: Receiver<char>, cx: &mut Context<AutomationView>) {
    let (tx, gpui_rx) = async_channel::unbounded::<String>();

    std::thread::spawn(move || {
        let mut state = StrokeCollector::default();

        loop {
            match rx.recv_timeout(Duration::from_millis(30)) {
                Ok('\n' | '\r') => {
                    if !state.is_empty() {
                        let _ = tx.try_send(std::mem::take(&mut state.collected_data));
                    }
                }
                Ok(ch) => state.push(ch),
                Err(RecvTimeoutError::Timeout) => state.clear(),
                Err(RecvTimeoutError::Disconnected) => break,
            }
        }
    });

    cx.spawn(|_view, cx: &mut AsyncApp| {
        let cx = cx.clone();
        async move {
            while let Ok(raw_barcode) = gpui_rx.recv().await {
                let _ = cx.update(|cx| {
                    cx.global_mut::<BillManager>()
                        .add_bill(raw_barcode, 0, 0.to_string());
                    cx.refresh_windows();
                });
            }
        }
    })
    .detach();
}
