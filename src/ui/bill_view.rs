use crate::states::{BillManager, LaserBill, ThemeManager};
use crate::ui::ternary;
use gpui::{App, Context, IntoElement, ParentElement, Render, RenderOnce, Styled, Window, div};
use gpui_component::{Icon, IconName};

impl RenderOnce for LaserBill {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<ThemeManager>();
        let icon = Icon::new(ternary(self.paid, IconName::Check, IconName::CircleCheck));
        let background = ternary(self.paid, theme.status_paid, theme.status_unpaid);

        div()
            .w_full()
            .flex()
            .bg(background)
            .child(icon)
            .child(self.reference.clone())
    }
}
