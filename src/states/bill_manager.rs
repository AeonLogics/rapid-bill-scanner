use chrono::{Local, NaiveDate};
use gpui::{App, Global, IntoElement};
use primitives::{LaserBill, TargetSoftware};
use win_ops::play_notification;

#[derive(Clone, Default, IntoElement)]
pub struct BillManager {
    pub bills: Vec<LaserBill>,
    pub active_reference: Option<String>,
}

impl Global for BillManager {}

impl BillManager {
    pub fn init(cx: &mut App) {
        cx.set_global(BillManager::default());
    }

    pub fn add_bill(&mut self, raw_code: String) {
        match LaserBill::parse(raw_code) {
            Some(bill) => {
                // if self.bill_exists(&bill.reference) {
                //     play_notification();
                //     return;
                // }
                self.bills.push(bill);
                if self.active_reference.is_none() {
                    self.select_next_unpaid();
                }
            }
            None => play_notification(),
        }
    }

    pub fn select_next_unpaid(&mut self) {
        self.active_reference = self
            .bills
            .iter()
            .find(|b| !b.paid)
            .map(|b| b.reference.clone());
    }

    pub fn get_active_bill(&self) -> Option<&LaserBill> {
        let active_ref = self.active_reference.as_ref()?;
        self.bills.iter().find(|b| &b.reference == active_ref)
    }

    pub fn mark_active_paid_and_advance(&mut self) {
        if let Some(active_ref) = self.active_reference.as_ref() {
            if let Some(bill) = self.bills.iter_mut().find(|b| &b.reference == active_ref) {
                bill.paid = true;
            }
        }
        self.select_next_unpaid();
    }

    pub fn execute_active(&mut self, target: TargetSoftware, contact: &str) -> bool {
        if let Some(bill) = self.get_active_bill() {
            target.execute(&bill.reference, contact);
            self.mark_active_paid_and_advance();
            true
        } else {
            false
        }
    }

    pub fn remove_bill(&mut self, reference: &str) {
        self.bills.retain(|b| b.reference != reference);
        if self.active_reference.as_deref() == Some(reference) {
            self.select_next_unpaid();
        }
    }

    pub fn clear_all(&mut self) {
        self.bills.clear();
        self.active_reference = None;
    }

    pub fn total_bills(&self) -> usize {
        self.bills.len()
    }

    pub fn total_unpaid_bills(&self) -> usize {
        self.bills.iter().filter(|b| !b.paid).count()
    }

    pub fn total_paid_bills(&self) -> usize {
        self.bills.iter().filter(|b| b.paid).count()
    }

    pub fn total_amount(&self) -> u32 {
        self.bills.iter().map(|b| b.amount).sum()
    }

    pub fn total_unpaid_amount(&self) -> u32 {
        self.bills
            .iter()
            .filter(|b| !b.paid)
            .map(|b| b.amount)
            .sum()
    }

    pub fn bill_exists(&self, reference: &str) -> bool {
        self.bills.iter().any(|b| b.reference == reference)
    }

    pub fn get(cx: &mut App) -> &mut BillManager {
        cx.global_mut::<BillManager>()
    }
}
