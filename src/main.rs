mod app;
mod component;
mod decks;
pub mod states;
mod ui;

use crate::app::RapidBillScanner;
use crate::states::{BillManager, Deck};
// use db_actions::init_db;
use gpui::{App, AppContext};
use primitives::{TargetSoftware, ThemeController};
use win_ops::LaserChannel;

pub fn main() {
    // init_db();
    gpui_platform::application()
        .with_assets(gpui_component_assets::Assets)
        .run(|cx: &mut App| {
            gpui_component::init(cx);
            ThemeController::init(cx);
            BillManager::init(cx);
            LaserChannel::init(cx);
            TargetSoftware::init(cx);
            Deck::init(cx);

            let ops = RapidBillScanner::window_options(cx);

            cx.spawn(async move |cx| {
                cx.open_window(ops, |_window, cx| cx.new(|_| RapidBillScanner {}))
                    .expect("Failed to open window");
            })
            .detach();
        });
}
