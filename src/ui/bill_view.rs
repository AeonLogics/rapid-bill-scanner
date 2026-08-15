use crate::states::{BillManager, LaserBill, ThemeManager};
use gpui::{
    App, InteractiveElement, IntoElement, ParentElement, RenderOnce, StatefulInteractiveElement,
    Styled, Window, div, px,
};
use gpui_component::button::Button;
use gpui_component::scroll::ScrollableElement;
use gpui_component::{Icon, IconName, StyledExt, h_flex, v_flex};

impl RenderOnce for LaserBill {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<ThemeManager>();
        let (status_color, status_text) = if self.paid {
            (theme.status_paid, "PAID")
        } else {
            (theme.status_unpaid, "UNPAID")
        };
        let reference = self.reference.clone();

        h_flex()
            .id(self.reference.clone())
            .w_full()
            .px_4()
            .on_click(move |_, _, cx| {
                let bills = BillManager::get(cx);
                bills.select_bill(reference.clone());
            })
            .py_2p5()
            .items_center()
            .border_b_1()
            .border_color(theme.border_color)
            .hover(|s| s.bg(theme.button_bg))
            .child(
                h_flex()
                    .w(px(140.0))
                    .gap_2()
                    .items_center()
                    .child(if let Some(idx) = self.index {
                        div()
                            .px_1p5()
                            .py_0p5()
                            .rounded_md()
                            .bg(theme.header_bg)
                            .text_xs()
                            .font_bold()
                            .text_color(theme.accent)
                            .child(format!("#{:02}", idx))
                    } else {
                        div()
                    })
                    .child(
                        div()
                            .text_xs()
                            .font_bold()
                            .text_color(theme.text_main)
                            .child(self.bill_type.to_uppercase()),
                    ),
            )
            // 2. Reference Column
            .child(
                div()
                    .flex_1()
                    .text_xs()
                    .text_color(theme.text_muted)
                    .child(format!("Ref: {}", self.reference.clone())),
            )
            // 3. Status Column
            .child(
                h_flex()
                    .w(px(140.0))
                    .gap_2()
                    .items_center()
                    .child(
                        div()
                            .text_xs()
                            .font_bold()
                            .text_color(status_color)
                            .child(status_text),
                    )
                    .child(if self.has_late_fee {
                        div()
                            .text_xs()
                            .font_medium()
                            .text_color(theme.status_unpaid)
                            .child("• Late Fee")
                    } else {
                        div()
                    }),
            )
            // 4. Amount Column
            .child(
                div()
                    .w(px(120.0))
                    .text_right()
                    .text_sm()
                    .font_bold()
                    .text_color(theme.text_main)
                    .child(format!("Rs. {}", self.amount)),
            )
            .child(
                div().w(px(64.0)).flex().justify_center().child(
                    Button::new(format!("delete-row-{}", self.reference))
                        .icon(IconName::Close)
                        .text_color(theme.status_unpaid)
                        .on_click({
                            let ref_code = self.reference.clone();
                            move |_event, _window, cx| {
                                cx.global_mut::<BillManager>().remove_bill(ref_code.clone());
                                cx.refresh_windows();
                            }
                        }),
                ),
            )
    }
}

impl RenderOnce for BillManager {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<ThemeManager>();
        let grand_total: u32 = self.bills.iter().map(|b| b.amount).sum();
        let total_count = self.bills.len();

        v_flex()
            .size_full()
            .bg(theme.panel_bg)
            .border_1()
            .border_color(theme.border_color)
            .rounded_xl()
            .overflow_hidden()
            // Table Header Bar
            .child(
                h_flex()
                    .w_full()
                    .px_4()
                    .py_3()
                    .bg(theme.header_bg)
                    .border_b_1()
                    .border_color(theme.border_color)
                    .text_xs()
                    .font_bold()
                    .text_color(theme.text_muted)
                    .child(div().w(px(140.0)).child("TYPE"))
                    .child(div().flex_1().child("REFERENCE"))
                    .child(div().w(px(140.0)).child("STATUS"))
                    .child(div().w(px(120.0)).text_right().child("AMOUNT"))
                    .child(div().w(px(64.0)).text_center().child("ACTIONS")),
            )
            // Scrollable Body
            .child(
                div()
                    .flex_1()
                    .min_h_0()
                    .w_full()
                    .overflow_y_scrollbar()
                    .child(if self.bills.is_empty() {
                        v_flex().size_full().items_center().justify_center().child(
                            div()
                                .text_sm()
                                .font_medium()
                                .text_color(theme.text_muted)
                                .child("No bills scanned in current batch"),
                        )
                    } else {
                        v_flex().w_full().children(
                            self.bills
                                .into_iter()
                                .enumerate()
                                .map(|(idx, bill)| bill.with_index(idx + 1)),
                        )
                    }),
            )
            // Table Footer Summary
            .child(
                h_flex()
                    .w_full()
                    .justify_between()
                    .items_center()
                    .px_4()
                    .py_3()
                    .bg(theme.header_bg)
                    .border_t_1()
                    .border_color(theme.border_color)
                    .child(
                        div()
                            .text_xs()
                            .font_bold()
                            .text_color(theme.text_muted)
                            .child(format!("BATCH: {} ITEMS", total_count)),
                    )
                    .child(
                        h_flex()
                            .gap_2()
                            .items_center()
                            .child(
                                div()
                                    .text_xs()
                                    .font_bold()
                                    .text_color(theme.text_muted)
                                    .child("GRAND TOTAL:"),
                            )
                            .child(
                                div()
                                    .text_base()
                                    .font_bold()
                                    .text_color(theme.accent)
                                    .child(format!("Rs. {}", grand_total)),
                            ),
                    ),
            )
    }
}
