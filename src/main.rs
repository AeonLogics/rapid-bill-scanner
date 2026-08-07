mod app;
mod theme_manager;
mod views;

// use-comamnds
use gpui::{App, AppContext, Entity};

use crate::{app::RapidBillScanner, theme_manager::ThemeManager, views::Header};

pub fn main() {
    gpui_platform::application()
        .with_assets(gpui_component_assets::Assets)
        .run(|cx: &mut App| {
            let ops = RapidBillScanner::window_options(cx);
            gpui_component::init(cx);
            ThemeManager::init(cx);
            cx.spawn(async move |cx| {
                cx.open_window(ops, |window, cx| {
                    let herader_view = cx.new(|cx| Header::new());
                })
                .expect("Failed to open window");
            })
            .detach();
        });
}
