use crate::states::{Deck, GlobalDeck};
use gpui::prelude::*;
use gpui::{App, IntoElement, MouseButton, RenderOnce, Styled, Window, div, px};
use gpui_component::{Icon, StyledExt, h_flex};
use primitives::ThemeController;

#[derive(IntoElement)]
pub struct DeckButton(pub Deck);

impl RenderOnce for DeckButton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<ThemeController>();
        let is_selected = GlobalDeck::active(cx) == self.0;
        let bg = if is_selected {
            theme.bg_sunken
        } else {
            gpui::rgba(0x00000000)
        };

        let content_color = if is_selected {
            theme.text_main
        } else {
            theme.text_muted
        };

        let border_color = if is_selected {
            theme.border_subtle
        } else {
            gpui::rgba(0x00000000)
        };

        let target_deck = self.0;

        h_flex()
            .w_full()
            .h(px(40.0))
            .px_4()
            .items_center()
            .justify_between()
            .rounded_xl()
            .bg(bg)
            .border_1()
            .border_color(border_color)
            .hover(|style| {
                if !is_selected {
                    style.bg(theme.bg_hover)
                } else {
                    style
                }
            })
            .cursor_pointer()
            .on_mouse_down(MouseButton::Left, move |_event, _window, cx| {
                GlobalDeck::set(cx, target_deck);
            })
            .child(
                h_flex()
                    .items_center()
                    .gap_3()
                    .child(Icon::new(self.0.icon()).size_4().text_color(content_color))
                    .child(
                        div()
                            .text_sm()
                            .font_semibold()
                            .text_color(content_color)
                            .child(self.0.to_string()),
                    ),
            )
            .when(is_selected, |this| {
                this.child(div().w(px(4.0)).h(px(16.0)).rounded_full().bg(theme.accent))
            })
    }
}
