use chrono::{Local, NaiveDate};
use std::fmt::{Display, Formatter};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BillType {
    Electric,
    // SuiGas,
}

impl Display for BillType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BillType::Electric => write!(f, "Electric"),
        }
    }
}

impl BillType {
    pub fn extract_bill(raw_data: String) -> Option<LaserBill> {
        let electric_string_size = 60;

        match raw_data.len() {
            electric_string_size => Self::extract_electricity_bill(raw_data),
            _ => None,
        }
    }

    fn extract_gas_bill(raw_data: String) -> Option<LaserBill> {
        let reference = raw_data.get(1..15).unwrap();
        None
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
        let is_late;

        if date_2 < today {
            amount = raw_data.get(amount_3)?.parse().ok()?;
            is_late = true;
        } else if date_1 < today {
            amount = raw_data.get(amount_2)?.parse().ok()?;
            is_late = true;
        } else {
            amount = raw_data.get(amount_1)?.parse().ok()?;
            is_late = false;
        }

        Some(LaserBill {
            bill_type: BillType::Electric,
            reference_number: reference.to_string(),
            amount,
            is_late,
            is_paid: false,
        })
    }
}

#[derive(Debug, Clone)]
pub struct LaserBill {
    pub bill_type: BillType,
    pub reference_number: String,
    pub amount: u32,
    pub is_late: bool,
    pub is_paid: bool,
}

#[derive(Default)]
pub struct LaserBillManager {
    pub bills: Vec<LaserBill>,
    pub active_index: usize,
}

impl LaserBillManager {
    pub const fn init() -> Self {
        Self {
            bills: Vec::new(),
            active_index: 0,
        }
    }

    pub fn add_bill(&mut self, bill: LaserBill) {
        self.bills.push(bill);
        self.active_index = self.bills.len().saturating_sub(1);
    }

    pub fn mark_last_paid(&mut self) {
        if let Some(last) = self.bills.last_mut() {
            last.is_paid = true;
        }
    }

    pub fn clear(&mut self) {
        self.bills.clear();
        self.active_index = 0;
    }

    pub fn total_amount(&self) -> u32 {
        self.bills.iter().map(|b| b.amount).sum()
    }
}
