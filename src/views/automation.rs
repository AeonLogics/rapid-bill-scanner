use crate::theme_manager::ThemeManager;
use gpui::{Context, IntoElement, Render, Window, div, prelude::*, px, rgb};
use gpui_component::StyledExt;
use gpui_component::button::Button;
use lasersink::{LaserExtractor, MemoryManager};

pub struct AutomationView {
    extractor: LaserExtractor,
    memory: MemoryManager,
}

impl AutomationView {
    pub fn new() -> AutomationView {
        AutomationView {
            extractor: LaserExtractor::new(),
            memory: MemoryManager::init(),
        }
    }
}

impl Render for AutomationView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<ThemeManager>();
        let label = if self.extractor.is_active() {
            "Stop"
        } else {
            "Start"
        };

        div().bg(theme.bg_app).size_full().p_3().child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .w_full()
                .bg(theme.panel_bg)
                .border_1()
                .border_color(theme.border_color)
                .rounded_lg()
                .p_2()
                .child(
                    // Status Badge with live colored dot
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .px_3()
                        .py_1()
                        .bg(theme.bg_app)
                        .border_1()
                        .border_color(theme.border_color)
                        .rounded_md()
                        // .child(div().size_2().rounded_full().bg())
                        .child(
                            div()
                                .text_sm()
                                .font_medium()
                                .text_color(theme.text_main)
                                .child(" Important information"),
                        ),
                )
                .child(Button::new(label).label(label).on_click(cx.listener(
                    |this, _event, _window, cx| {
                        this.extractor.toggle();
                        cx.notify();
                    },
                ))),
        )
    }
}
