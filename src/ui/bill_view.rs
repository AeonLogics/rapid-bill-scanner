use crate::theme_manager::ThemeManager;
use crate::ui::ternary;
use gpui::{App, Context, IntoElement, ParentElement, RenderOnce, Styled, Window, div};
use lasersink::LaserBill;

#[derive(IntoElement)]
pub struct BillView {
    bill: LaserBill,
}

impl BillView {
    pub fn init(bill: LaserBill) -> Self {
        Self { bill }
    }
}

impl RenderOnce for BillView {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<ThemeManager>();

        div()
            .w_full()
            .border_1()
            .border_color(theme.border_color)
            .bg(ternary(
                self.bill.is_paid,
                theme.status_paid,
                theme.status_unpaid,
            ))
            .child(self.bill.reference_number)
    }
}
