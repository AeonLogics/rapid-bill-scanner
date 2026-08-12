use gpui::{Context, IntoElement, Render, Window};

pub struct MessageDisplay {
    title: String,
    message: String,
}

impl Render for MessageDisplay {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {}
}
