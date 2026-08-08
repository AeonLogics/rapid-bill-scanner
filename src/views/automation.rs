use gpui::{Window, div, prelude::*};

pub struct AutomationView {
    counter: usize,
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
