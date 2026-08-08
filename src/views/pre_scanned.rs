use gpui::{Render, Window};
use gpui::{div, prelude::*};

pub struct PreScannedView {
    // pub scanned_bills: Vec,
}

impl PreScannedView {
    pub fn new() -> Self {
        Self {}
    }
}

impl Render for PreScannedView {
    fn render(&mut self, _: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().child("I am Pre Scanned")
    }
}
