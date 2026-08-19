use crate::core::target_software::TargetSoftware;
use chrono::{Local, NaiveDate};

#[derive(Clone, Debug)]
pub struct LaserBill {
    pub index: Option<usize>,
    pub bill_type: &'static str,
    pub reference: String,
    pub amount: u32,
    pub has_late_fee: bool,
    pub paid: bool,
}

impl LaserBill {
    pub fn parse(raw_code: String) -> Option<LaserBill> {
        if raw_code.len() == 60 {
            extract_electricity_bill(raw_code)
        } else {
            None
        }
    }

    pub fn with_index(mut self, index: usize) -> Self {
        self.index = Some(index);
        self
    }

    pub fn execute(self, target: TargetSoftware, contact: &str) -> bool {
        target.execute(&self.reference, contact);
        true
    }

    pub fn execute_empty(target: TargetSoftware, contact: &str) -> bool {
        target.execute("", contact);
        true
    }
}

fn extract_electricity_bill(raw_data: String) -> Option<LaserBill> {
    if raw_data.len() < 60 {
        return None;
    }

    let today = Local::now().date_naive();

    let reference = raw_data.get(1..15)?;
    let date_1 = NaiveDate::parse_from_str(raw_data.get(19..25)?, "%d%m%y").ok()?;
    let date_2 = NaiveDate::parse_from_str(raw_data.get(43..49)?, "%d%m%y").ok()?;

    let (amount_str, has_late_fee) = if date_2 < today {
        (raw_data.get(35..43)?, true)
    } else if date_1 < today {
        (raw_data.get(50..58)?, true)
    } else {
        (raw_data.get(26..34)?, false)
    };

    let amount = amount_str.parse().ok()?;

    Some(LaserBill {
        index: None,
        bill_type: "Electricity",
        reference: reference.to_string(),
        amount,
        has_late_fee,
        paid: false,
    })
}

// impl RenderOnce for LaserBill {
//     fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
//         let theme = cx.global::<ThemeController>();
//         let (status_color, status_text) = if self.paid {
//             (theme.status_paid, "PAID")
//         } else {
//             (theme.status_unpaid, "UNPAID")
//         };
//         let reference = self.reference.clone();
//
//         h_flex()
//             .id(self.reference.clone())
//             .w_fill()
//             .px_4()
//             // .on_click(move |_, _, cx| {
//             //     let bills = BillManager::get(cx);
//             //     bills.select_bill(reference.clone());
//             // })
//             .py_2p5()
//             .items_center()
//             .border_b_1()
//             .border_color(theme.border_color)
//             .hover(|s| s.bg(theme.button_bg))
//             .child(
//                 h_flex()
//                     .w(px(140.0))
//                     .gap_2()
//                     .items_center()
//                     .child(if let Some(idx) = self.index {
//                         div()
//                             .px_1p5()
//                             .py_0p5()
//                             .rounded_md()
//                             .bg(theme.header_bg)
//                             .text_xs()
//                             .font_bold()
//                             .text_color(theme.accent)
//                             .child(format!("#{:02}", idx))
//                     } else {
//                         div()
//                     })
//                     .child(
//                         div()
//                             .text_xs()
//                             .font_bold()
//                             .text_color(theme.text_main)
//                             .child(self.bill_type.to_uppercase()),
//                     ),
//             )
//             // 2. Reference Column
//             .child(
//                 div()
//                     .flex_1()
//                     .text_xs()
//                     .text_color(theme.text_muted)
//                     .child(format!("Ref: {}", self.reference.clone())),
//             )
//             // 3. Status Column
//             .child(
//                 h_flex()
//                     .w(px(140.0))
//                     .gap_2()
//                     .items_center()
//                     .child(
//                         div()
//                             .text_xs()
//                             .font_bold()
//                             .text_color(status_color)
//                             .child(status_text),
//                     )
//                     .child(if self.has_late_fee {
//                         div()
//                             .text_xs()
//                             .font_medium()
//                             .text_color(theme.status_unpaid)
//                             .child("• Late Fee")
//                     } else {
//                         div()
//                     }),
//             )
//             // 4. Amount Column
//             .child(
//                 div()
//                     .w(px(120.0))
//                     .text_right()
//                     .text_sm()
//                     .font_bold()
//                     .text_color(theme.text_main)
//                     .child(format!("Rs. {}", self.amount)),
//             )
//             .child(
//                 div().w(px(64.0)).flex().justify_center().child(
//                     Button::new(format!("delete-row-{}", self.reference))
//                         .icon(IconName::Close)
//                         .text_color(theme.status_unpaid)
//                         .on_click({
//                             let ref_code = self.reference.clone();
//                             move |_event, _window, cx| {
//                                 cx.global_mut::<BillManager>().remove_bill(&ref_code);
//                                 cx.refresh_windows();
//                             }
//                         }),
//                 ),
//             )
//     }
// }
