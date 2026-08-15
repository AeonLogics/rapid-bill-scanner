use crate::states::ThemeManager;
use chrono::{Local, NaiveDate};
use gpui::{
    App, Context, Global, IntoElement, ParentElement, RenderOnce, Styled, Window, div, rgb,
};
use gpui_component::{StyledExt, h_flex, v_flex};
use lasersink::play_bonk_error;

impl Global for BillManager {}

#[derive(IntoElement, Clone)]
pub struct LaserBill {
    pub index: Option<usize>,
    pub bill_type: &'static str,
    pub reference: String,
    pub amount: u32,
    pub has_late_fee: bool,
    pub paid: bool,
}

impl LaserBill {
    pub fn parse(raw_code: String, amount: u32, reference: String) -> Option<LaserBill> {
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
}

#[derive(IntoElement, Clone)]
pub struct BillManager {
    pub bills: Vec<LaserBill>,
    pub active_reference: Option<String>,
}

impl BillManager {
    pub fn init_memory(cx: &mut App) {
        let bill = BillManager {
            bills: Vec::new(),
            active_reference: None,
        };
        // for amount in 1..100 {
        //     bill.add_bill(
        //         "E1115437303031807260408260000125500000135910708260000130716E".to_string(),
        //         amount,
        //         amount.to_string(),
        //     )
        // }
        cx.set_global(bill);
    }

    pub fn add_bill(&mut self, raw_code: String, amount: u32, reference: String) {
        let bill = LaserBill::parse(raw_code, amount, reference);
        match bill {
            Some(bill) => {
                self.bills.push(bill);
            }
            None => {
                println!("Unable to extract Bill BONK");
                play_bonk_error();
            }
        }
    }

    pub fn remove_bill(&mut self, reference: String) {
        self.bills.retain(|b| b.reference != reference);
    }

    pub fn clear_all(&mut self) {
        self.bills.clear();
        self.active_reference = None;
    }

    pub fn select_bill(&mut self, reference: String) -> bool {
        if let Some(bill) = self.bills.iter().find(|b| b.reference == reference) {
            if !bill.paid {
                self.active_reference = Some(reference);
                return true;
            }
        }
        false
    }

    pub fn get(cx: &mut App) -> &mut Self {
        cx.global_mut::<Self>()
    }
}

fn extract_electricity_bill(raw_data: String) -> Option<LaserBill> {
    let reference = 1..15;
    let date_1 = 19..25;
    let date_2 = 43..49;
    let amount_1 = 26..34;
    let amount_2 = 50..58;
    let amount_3 = 35..43;

    let today = Local::now().date_naive();

    let reference = raw_data.get(reference)?;
    let date_1 = NaiveDate::parse_from_str(raw_data.get(date_1)?, "%d%m%y").ok()?;
    let date_2 = NaiveDate::parse_from_str(raw_data.get(date_2)?, "%d%m%y").ok()?;

    let amount: u32;
    let has_late_fee;

    if date_2 < today {
        amount = raw_data.get(amount_3)?.parse().ok()?;
        has_late_fee = true;
    } else if date_1 < today {
        amount = raw_data.get(amount_2)?.parse().ok()?;
        has_late_fee = true;
    } else {
        amount = raw_data.get(amount_1)?.parse().ok()?;
        has_late_fee = false;
    }

    Some(LaserBill {
        index: None,
        bill_type: "Electricity",
        reference: reference.to_string(),
        amount,
        has_late_fee,
        paid: false,
    })
}
