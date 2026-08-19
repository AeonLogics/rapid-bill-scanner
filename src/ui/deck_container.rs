use crate::component::DeckTitleBar;
use crate::states::Deck;
use gpui::{App, IntoElement, ParentElement, RenderOnce, Styled, Window, div};
use gpui_component::v_flex;

#[derive(IntoElement)]
pub struct DeckContainer;

impl RenderOnce for DeckContainer {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let deck = *cx.global::<Deck>();

        v_flex()
            .flex_1()
            .h_full()
            .pl_2p5()
            .overflow_hidden()
            .child(DeckTitleBar)
            .child(div().pt_2p5().size_full().child(deck))
    }
}
