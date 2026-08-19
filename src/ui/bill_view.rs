use crate::states::BillManager;
use gpui::{
    App, InteractiveElement, IntoElement, ParentElement, RenderOnce, StatefulInteractiveElement,
    Styled, Window, div, px,
};
use gpui_component::scroll::ScrollableElement;
use gpui_component::{Icon, IconName, StyledExt, h_flex, v_flex};
use primitives::ThemeController;

impl RenderOnce for BillManager {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<ThemeController>();
        let grand_total: u32 = self.bills.iter().map(|b| b.amount).sum();
        let total_count = self.bills.len();

        v_flex()
            .size_full()
            .bg(theme.bg_panel) // theme.panel_bg -> theme.bg_panel
            .border_1()
            .border_color(theme.border) // theme.border_color -> theme.border
            .rounded_xl()
            .overflow_hidden()
            // Table Header Bar
            .child(
                h_flex()
                    .w_full()
                    .px_4()
                    .py_3()
                    .bg(theme.bg_surface) // theme.header_bg -> theme.bg_surface
                    .border_b_1()
                    .border_color(theme.border)
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
                                .map(|(_idx, bill)| div().child(bill.reference)),
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
                    .bg(theme.bg_surface) // theme.header_bg -> theme.bg_surface
                    .border_t_1()
                    .border_color(theme.border)
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
