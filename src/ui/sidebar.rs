use crate::component::{DockHeader, SystemDeck};
use crate::states::GlobalDeck;
use gpui::prelude::*;
use gpui::{App, Entity, IntoElement, Styled, Window, div, px};
use gpui_component::{StyledExt, v_flex};

#[derive(Clone)]
pub struct ControlRoom;

impl ControlRoom {
    pub fn build(cx: &mut App) -> Entity<Self> {
        cx.new(|cx| {
            let deck_handle = GlobalDeck::handle(cx);
            cx.observe(&deck_handle, |_, _view, cx| cx.notify())
                .detach();
            Self
        })
    }
}

impl Render for ControlRoom {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .h_full()
            .w(px(320.0))
            .overflow_hidden()
            .child(DockHeader)
            .child(div().pt_2p5().size_full().child(SystemDeck))
    }
}
