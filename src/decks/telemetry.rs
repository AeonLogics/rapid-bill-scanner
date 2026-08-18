use crate::states::{LaserBill, TargetDropdown, ThemeManager};
use crate::ui::ternary;
use gpui::{Context, IntoElement, Render, Window, div, prelude::*, px};
use gpui_component::{IconName, StyledExt, h_flex, v_flex};
use lasersink::LaserChannel;

pub struct TelemetryDeck {
    pub transmit_channel: LaserChannel,
    last_scanned: Option<LaserBill>,
}

impl TelemetryDeck {
    pub fn new() -> Self {
        Self {
            transmit_channel: LaserChannel::init(),
            last_scanned: None,
        }
    }
}

impl Render for TelemetryDeck {
    fn render(&mut self, win: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<ThemeManager>();
        let is_running = self.transmit_channel.status();
        let target_software = cx.global::<TargetDropdown>();

        if is_running {
            cx.on_next_frame(win, |_, _, cx| {
                cx.notify();
            });
        }

        h_flex().bg(theme.bg_app).size_full().p_4().gap_4().child(
            h_flex()
                .flex_1()
                .h_full()
                .gap_4()
                // .child(ScanMonitor::new(is_running))
                .child(
                    v_flex()
                        .w(px(320.0))
                        .h_full()
                        .p_6()
                        .bg(theme.panel_bg)
                        .border_1()
                        .border_color(theme.border_color)
                        .rounded_xl()
                        .justify_between()
                        .child(target_software.clone())
                        .child(
                            div()
                                .id("toggle_btn")
                                .w_full()
                                .py_3()
                                .px_4()
                                .rounded_lg()
                                .flex()
                                .items_center()
                                .justify_center()
                                .gap_2()
                                .font_bold()
                                .text_base()
                                .cursor_pointer()
                                .bg(theme.accent)
                                .text_color(theme.text_on_accent)
                                .hover(|s| s.bg(theme.accent_hover))
                                .active(|s| s.bg(theme.accent_active))
                                .child(gpui_component::Icon::new(ternary(
                                    is_running,
                                    IconName::Pause,
                                    IconName::Play,
                                )))
                                .child(ternary(!is_running, "Activate Engine", "Deactivate Engine"))
                                .on_click(cx.listener(|this, _event, _window, cx| {
                                    this.toggle_automation(cx);
                                })),
                        ),
                ),
        )
    }
}
