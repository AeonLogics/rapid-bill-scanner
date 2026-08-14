use crate::states::ThemeManager;
use crate::views::{Header, TabView};
use gpui::{App, Bounds, Entity, Window, WindowBounds, WindowOptions, px, size};
use gpui::{div, prelude::*};

pub struct RapidBillScanner {
    pub header: Entity<Header>,
    pub tab_view: Entity<TabView>,
}

impl RapidBillScanner {
    pub fn bounds(cx: &mut App) -> WindowBounds {
        let bounds = Bounds::centered(None, size(px(1280.), px(720.0)), cx);
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
        let theme = cx.global::<ThemeManager>();

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(theme.bg_app)
            .text_color(theme.text_main)
            .child(self.header.clone())
            .child(
                div()
                    .flex_1()
                    .w_full()
                    .child(self.tab_view.clone())
                    .size_full(),
            )
    }
}
