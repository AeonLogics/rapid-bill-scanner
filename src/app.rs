use crate::states::Deck;
use crate::ui::{ControlRoom, DeckContainer};
use gpui::{
    App, Bounds, SharedString, TitlebarOptions, Window, WindowBounds, WindowOptions, px, size,
};
use gpui::{div, prelude::*};
use primitives::ThemeController;

pub struct RapidBillScanner {}

impl RapidBillScanner {
    pub fn bounds(cx: &mut App) -> WindowBounds {
        let bounds = Bounds::centered(None, size(px(1280.), px(800.0)), cx);
        let bounds = Bounds::centered(None, size(px(1280.), px(800.0)), cx);
        WindowBounds::Windowed(bounds)
    }

    pub fn window_options(cx: &mut App) -> WindowOptions {
        let win_size = Self::bounds(cx);
        WindowOptions {
            window_bounds: Some(win_size),
            titlebar: None,
            ..Default::default()
        }
    }
}

impl Render for RapidBillScanner {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<ThemeController>();
        let deck = cx.global::<Deck>();

        div()
            .size_full()
            .bg(theme.bg_app)
            .flex()
            .flex_row()
            .overflow_hidden()
            .child(ControlRoom)
            .child(DeckContainer)
    }
}
