use crate::component::{DeckTitleBar, DockHeader, SystemDeck};
use gpui::prelude::*;
use gpui::{App, IntoElement, RenderOnce, Styled, Window, div, px, red};
use gpui_component::{StyledExt, v_flex};
use primitives::ThemeController;

#[derive(Default, IntoElement)]
pub struct ControlRoom;

impl RenderOnce for ControlRoom {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        v_flex()
            .h_full()
            .w(px(320.0))
            .overflow_hidden()
            .child(DockHeader)
            .child(div().pt_2p5().size_full().child(SystemDeck))
    }
}
