use chrono::{Local, NaiveDate};
use gpui::{App, Context, Global, IntoElement};
use lasersink::play_bonk_error;

impl Global for BillManager {}

#[derive(IntoElement)]
pub struct LaserBill {
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
}

pub struct BillManager {
    bills: Vec<LaserBill>,
    index: Option<u32>,
}

impl BillManager {
    pub fn init_memory(cx: &mut App) {
        let bill = BillManager {
            bills: Vec::new(),
            index: None,
        };
        cx.set_global(bill);
    }

    pub fn add_bill(&mut self, raw_code: String) {
        let bill = LaserBill::parse(raw_code);
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
}

fn extract_electricity_bill(raw_data: String) -> Option<LaserBill> {
    let reference = 1..15;
    let date_1 = 19..25;
    let date_2 = 43..49;
    let amount_1 = 26..34;
    let amount_2 = 50..58;
    let amount_3 = 35..43;

    // today
    let today = Local::now().date_naive();

    // converting ranges to data
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
        bill_type: "Electricity",
        reference: reference.to_string(),
        amount,
        has_late_fee,
        paid: false,
    })
}
