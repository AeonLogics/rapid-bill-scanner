use crate::component::DropDown;
use crate::ui::{ProceduralScanner, ternary};
use gpui::prelude::*;
use gpui::{App, Context, Entity, IntoElement, Render, Window, div, px};
use gpui_component::{Icon, IconName, StyledExt, h_flex, v_flex};
use primitives::{TargetSoftware, ThemeController};
use win_ops::LaserChannel;

pub struct TelemetryDeck;

impl TelemetryDeck {
    pub fn build(cx: &mut App) -> Entity<Self> {
        cx.new(|_| Self)
    }
}

impl Render for TelemetryDeck {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<ThemeController>();
        let channel = cx.global::<LaserChannel>();
        let is_running = channel.status();
        let last_bill = channel.last_bill();

        v_flex()
            .size_full()
            .gap_4()
            .p_4()
            .rounded_tl_2xl()
            .bg(theme.bg_surface)
            .child(ProceduralScanner::new(is_running, last_bill))
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
            // Bottom Section: Main Action Trigger
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
                            .on_click(|_event, _window, cx| {
                                cx.update_global::<LaserChannel, _>(|channel, cx| {
                                    channel.toggle(TargetSoftware::Nadra);
                                    // cx.notify();
                                });
                            }),
                    ),
            )
    }
}
