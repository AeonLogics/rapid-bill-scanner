use gpui::{Context, IntoElement, ParentElement, Render, Window, div};

pub struct Header {}

impl Render for Header {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Header>) -> impl IntoElement {
        div().child("Header")
    }
}

impl Header {
    pub fn new() -> Self {
        Self {}
    }
}
