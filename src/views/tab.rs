use super::{AutomationView, PreScannedView};

use gpui::{
    App, AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div,
};

pub struct TabView {
    pub active_tab: usize,
    automaton_view: Entity<AutomationView>,
    pre_scanned_view: Entity<PreScannedView>,
}

impl TabView {
    pub fn init(cx: &mut App) -> Self {
        Self {
            active_tab: 0,
            automaton_view: cx.new(|_cx| AutomationView::new()),
            pre_scanned_view: cx.new(|_cx| PreScannedView::new()),
        }
    }

    pub fn set_tab(&mut self, index: usize, cx: &mut Context<Self>) {
        if self.active_tab != index {
            self.active_tab = index;
            cx.notify();
        }
    }
}

impl Render for TabView {
    fn render(&mut self, _: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        match self.active_tab {
            0 => div().size_full().child(self.automaton_view.clone()),
            1 => div().size_full().child(self.pre_scanned_view.clone()),
            _ => div().size_full().child("FAAAAAAAAAAAAAAAAAAAAAAAAAAAWh"),
        }
    }
}
