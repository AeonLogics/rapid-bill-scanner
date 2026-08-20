mod app;
mod component;
mod decks;
pub mod states;
mod ui;

use crate::app::RapidBillScanner;
use crate::states::{BillManager, GlobalDeck};
// use db_actions::init_db;
use crate::ui::ControlRoom;
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
            GlobalDeck::init(cx);

            let ops = RapidBillScanner::window_options(cx);

            cx.spawn(async move |cx| {
                cx.open_window(ops, |_win, cx| {
                    let control_room = ControlRoom::build(cx);
                    cx.new(|_cx| RapidBillScanner { control_room })
                })
            })
            .detach();
        });
}
