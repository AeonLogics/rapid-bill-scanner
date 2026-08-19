use crate::component::DropDown;
use crate::ui::ternary;
use gpui::prelude::*;
use gpui::{App, Global, IntoElement, RenderOnce, Window, div, px};
use gpui_component::{Icon, IconName, StyledExt, h_flex, v_flex};
use primitives::{TargetSoftware, ThemeController};
use win_ops::LaserChannel;

#[derive(Default, IntoElement)]
pub struct TelemetryDeck;

impl RenderOnce for TelemetryDeck {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<ThemeController>();
        let channel = cx.global::<LaserChannel>();
        let is_running = channel.status();

        v_flex()
            .size_full()
            .gap_4()
            .p_4()
            .rounded_tl_2xl()
            .bg(theme.bg_surface)
            .child(
                v_flex()
                    .w_full()
                    .p_5()
                    .bg(theme.bg_panel)
                    .border_1()
                    .border_color(theme.border_subtle)
                    .rounded_xl()
                    .gap_3()
                    .child(
                        h_flex()
                            .justify_between()
                            .items_center()
                            .child(
                                h_flex()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        // Dynamic pulsing status dot
                                        div().size_2p5().rounded_full().bg(ternary(
                                            is_running,
                                            theme.success,
                                            theme.text_muted,
                                        )),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_semibold()
                                            .text_color(theme.text_main)
                                            .child(ternary(
                                                is_running,
                                                "ENGINE LIVE",
                                                "ENGINE IDLE",
                                            )),
                                    ),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text_muted)
                                    .child("CHANNEL 01 — ACTIVE"),
                            ),
                    )
                    // Visualizer track bar / animation placeholder
                    .child(
                        h_flex()
                            .w_full()
                            .h(px(40.0))
                            .bg(theme.bg_sunken)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .rounded_lg()
                            .items_center()
                            .justify_center()
                            .child(div().text_xs().text_color(theme.text_muted).child(ternary(
                                is_running,
                                "Scanning optical stream...",
                                "Standby mode",
                            ))),
                    ),
            )
            // 2. Middle Section: Target & Configuration Controls
            .child(
                v_flex()
                    .w_full()
                    .p_5()
                    .bg(theme.bg_panel)
                    .border_1()
                    .border_color(theme.border_subtle)
                    .rounded_xl()
                    .gap_4()
                    .child(
                        div()
                            .text_xs()
                            .font_bold()
                            .text_color(theme.text_muted)
                            .child("TARGET CONFIGURATION"),
                    )
                    // Dropdown & parameter row
                    .child(
                        h_flex()
                            .w_full()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(theme.text_main)
                                    .child("Software Target"),
                            )
                            // Target Dropdown placeholder
                            .child(
                                div()
                                    .px_3()
                                    .py_1p5()
                                    .bg(theme.bg_surface)
                                    .border_1()
                                    .border_color(theme.border_subtle)
                                    .rounded_md()
                                    .text_xs()
                                    .text_color(theme.text_main)
                                    .child("NADRA System"),
                            ),
                    ),
            )
            // 3. Bottom Section: Main Action Trigger
            .child(
                v_flex()
                    .w_full()
                    .p_4()
                    .bg(theme.bg_panel)
                    .border_1()
                    .border_color(theme.border_subtle)
                    .rounded_xl()
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
                            .text_sm()
                            .cursor_pointer()
                            .bg(ternary(is_running, theme.error, theme.accent))
                            .text_color(theme.text_on_accent)
                            .hover(|s| s.bg(ternary(is_running, theme.red, theme.accent_hover)))
                            .active(|s| s.bg(theme.accent_active))
                            .child(Icon::new(ternary(
                                is_running,
                                IconName::Pause,
                                IconName::Play,
                            )))
                            .child(ternary(!is_running, "Activate Engine", "Deactivate Engine"))
                            .on_click(|_event, _window, cx| {}),
                    ),
            )
    }
}
