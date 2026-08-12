mod app;
mod theme_manager;
mod ui;
mod views;
use gpui::{App, AppContext};
use gpui_component::Root;

use crate::{
    app::RapidBillScanner,
    theme_manager::ThemeManager,
    views::{Header, TabView},
};

pub fn main() {
    gpui_platform::application()
        .with_assets(gpui_component_assets::Assets)
        .run(|cx: &mut App| {
            gpui_component::init(cx);
            ThemeManager::init(cx);

            let ops = RapidBillScanner::window_options(cx);

            cx.spawn(async move |cx| {
                cx.open_window(ops, |window, cx| {
                    let tab_view = cx.new(|cx| TabView::init(cx));
                    let header_view = cx.new(|cx| Header::new(tab_view.clone(), cx));
                    let app_view = cx.new(|_| RapidBillScanner {
                        header: header_view,
                        tab_view,
                    });
                    cx.new(|cx| Root::new(app_view, window, cx))
                })
                .expect("Failed to open window");
            })
            .detach();
        });
}

// fn main() {
//     // let rezsult = core::change_computer_name("Aeon Roamer".to_string());
//     core::register_raw_input_listener();
// }
