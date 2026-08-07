use gpui::{Context, IntoElement, ParentElement, Render, Window, div};

pub enum Tab {
    Automation,
    PreScanned,
    Settings,
}

impl Render for Tab {
    fn render(&mut self, _: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        match self {
            Tab::Automation => div().child("Automation"),
            Tab::PreScanned => div().child("PreScanned"),
            Tab::Settings => div().child("Settings"),
        }
    }
}

impl Default for Tab {
    fn default() -> Self {
        Tab::Automation
    }
}

impl Tab {}
