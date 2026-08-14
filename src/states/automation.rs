use crate::states::BillManager;
use crate::views::AutomationView;
use gpui::Context;

fn toggle_automation_thread(cx: &mut Context<AutomationView>) {
    cx.spawn(async move |_view, cx| {
        let cx =
    }).detach();
}
