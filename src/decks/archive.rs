use gpui::{App, AppContext, Context, Entity, IntoElement, ParentElement, Render, Window, div};

pub struct ArchiveDeck;

impl ArchiveDeck {
    pub fn build(cx: &mut App) -> Entity<Self> {
        cx.new(|_cx| ArchiveDeck)
    }
}

impl Render for ArchiveDeck {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().child("I am Archive")
    }
}
