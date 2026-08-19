use crate::component::deck_button::DeckButton;
use crate::states::Deck;
use gpui::prelude::*;
use gpui::{App, IntoElement, RenderOnce, Window, div};
use gpui_component::{StyledExt, v_flex};
use primitives::ThemeController;

#[derive(Default, IntoElement)]
pub struct SystemDeck;

impl RenderOnce for SystemDeck {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<ThemeController>();

        v_flex()
            .w_full()
            .h_full()
            .px_3()
            .rounded_tr_2xl()
            .py_2()
            .bg(theme.bg_surface)
            .gap_2()
            .child(
                div()
                    .px_3()
                    .pt_2()
                    .pb_1()
                    .text_xs()
                    .font_bold()
                    .text_color(theme.text_muted)
                    .child("DECK CONTROL"),
            )
            .child(DeckButton(Deck::Telemetry))
    }
}
