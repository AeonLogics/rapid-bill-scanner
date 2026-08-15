use crate::states::{BillManager, ThemeManager};
use crate::ui::ternary;
use gpui::{App, Context, IntoElement, Render, Window, div, prelude::*, px};
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::{IconName, StyledExt, h_flex, v_flex};
use lasersink::LaserChannel;

pub struct AutomationView {
    pub transmit_channel: LaserChannel,
}

impl AutomationView {
    pub fn new() -> AutomationView {
        AutomationView {
            transmit_channel: LaserChannel::init(),
        }
    }
}

impl Render for AutomationView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<ThemeManager>();
        let is_running = self.transmit_channel.status();

        let bill_manager = cx.global::<BillManager>();
        let grand_total: u32 = bill_manager.bills.iter().map(|b| b.amount).sum();
        let total_count = bill_manager.bills.len();

        h_flex()
            .bg(theme.bg_app)
            .size_full()
            .p_4()
            .gap_4()
            .child(div().flex_1().h_full().child(bill_manager.clone()))
            .child(
                v_flex()
                    .w(px(280.0))
                    .h_full()
                    .gap_3()
                    .child(
                        v_flex()
                            .w_full()
                            .p_4()
                            .bg(theme.panel_bg)
                            .border_1()
                            .border_color(theme.border_color)
                            .rounded_xl()
                            .gap_3()
                            .child(
                                div()
                                    .text_xs()
                                    .font_bold()
                                    .text_color(theme.text_muted)
                                    .child("AUTOMATION ENGINE"),
                            )
                            .child(
                                h_flex()
                                    .items_center()
                                    .gap_2()
                                    .child(div().size_2p5().rounded_full().bg(ternary(
                                        is_running,
                                        theme.accent,
                                        theme.text_muted,
                                    )))
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_semibold()
                                            .text_color(theme.text_main)
                                            .child(ternary(
                                                !is_running,
                                                "Laser Scanner is Inactive",
                                                "Laser Scanner is Active",
                                            )),
                                    ),
                            )
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
                                    .bg(theme.accent)
                                    .text_color(theme.text_on_accent)
                                    .hover(|s| s.bg(theme.accent_hover))
                                    .active(|s| s.bg(theme.accent_active))
                                    .child(gpui_component::Icon::new(ternary(
                                        is_running,
                                        IconName::Pause,
                                        IconName::Play,
                                    )))
                                    .child(ternary(!is_running, "Activate", "Deactivate"))
                                    .on_click(cx.listener(|this, _event, _window, cx| {
                                        this.toggle_automation(cx);
                                    })),
                            )
                            .child(
                                div()
                                    .id("clear-all-button")
                                    .w_full()
                                    .p_4()
                                    .rounded_lg()
                                    .border_1()
                                    .border_color(theme.border_color)
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .font_bold()
                                    .text_sm()
                                    .cursor_pointer()
                                    .bg(theme.panel_bg)
                                    .text_color(theme.red)
                                    .hover(|s| s.bg(theme.red).text_color(theme.text_on_accent))
                                    .child("Clear Batch")
                                    .on_click(|_event, _window, cx| {
                                        cx.global_mut::<BillManager>().clear_all();
                                        cx.refresh_windows();
                                    }),
                            ),
                    )
                    // 2. Batch Stat Card
                    .child(
                        v_flex()
                            .w_full()
                            .p_4()
                            .bg(theme.panel_bg)
                            .border_1()
                            .border_color(theme.border_color)
                            .rounded_xl()
                            .gap_2()
                            .child(
                                div()
                                    .text_xs()
                                    .font_bold()
                                    .text_color(theme.text_muted)
                                    .child("SCANNED BATCH"),
                            )
                            .child(
                                div()
                                    .text_2xl()
                                    .font_bold()
                                    .text_color(theme.text_main)
                                    .child(format!("{} Bills", total_count)),
                            ),
                    )
                    // 3. Grand Total Card
                    .child(
                        v_flex()
                            .w_full()
                            .p_4()
                            .bg(theme.panel_bg)
                            .border_1()
                            .border_color(theme.border_color)
                            .rounded_xl()
                            .gap_2()
                            .child(
                                div()
                                    .text_xs()
                                    .font_bold()
                                    .text_color(theme.text_muted)
                                    .child("GRAND TOTAL"),
                            )
                            .child(
                                div()
                                    .text_2xl()
                                    .font_bold()
                                    .text_color(theme.accent)
                                    .child(format!("Rs. {}", grand_total)),
                            ),
                    ),
            )
    }
}
