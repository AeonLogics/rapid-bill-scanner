use gpui::{div, prelude::*, Window};

pub struct AutomationView {
    counter: usize,
    total_bills_processed: usize,
    device: multiinput::DevicesDisplayInfo,
    // devices: Vec<Device>,
}

impl AutomationView {
    pub fn new() -> AutomationView {
        AutomationView { counter: 0 }
    }
}

impl Render for AutomationView {
    fn render(&mut self, _: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().child("MAA CHIAAAA")
    }
}
