mod app;
mod component;
pub mod states;
mod ui;
mod views;

use gpui::{App, AppContext};
use gpui_component::Root;

use crate::states::BillManager;
use crate::{
    app::RapidBillScanner,
    states::ThemeManager,
    views::{Header, TabView},
};

pub fn main() {
    gpui_platform::application()
        .with_assets(gpui_component_assets::Assets)
        .run(|cx: &mut App| {
            gpui_component::init(cx);
            ThemeManager::init(cx);
            BillManager::init_memory(cx);

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
