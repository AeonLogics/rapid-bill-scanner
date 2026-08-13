use crate::bills::{BillType, LaserBill, LaserBillManager};
use crate::core::target::TargetSoftware;
use crate::core::window::play_bonk_error;
use std::sync::Mutex;

static BILL_MEMORY: Mutex<LaserBillManager> = Mutex::new(LaserBillManager::init());
static TARGET_SOFTWARE_MEMORY: Mutex<TargetSoftware> = Mutex::new(TargetSoftware::init());
pub struct MemoryManager;

impl MemoryManager {
    pub fn init() -> MemoryManager {
        MemoryManager
    }

    pub fn add_bill(&self, raw_code: String) {
        if let Some(bill) = BillType::extract_bill(raw_code) {
            println!("Bill code: {}", bill.reference_number);
            BILL_MEMORY
                .lock()
                .expect("Unable to fetch Bills Lock")
                .add_bill(bill);
        } else {
            println!("Unable to extract Bill BONK");
            play_bonk_error();
        }
    }

    pub fn get_bills(&self) -> Vec<LaserBill> {
        BILL_MEMORY
            .lock()
            .expect("Unable to fetch Bills Lock")
            .bills
            .clone()
    }

    pub fn clear(&self) {
        BILL_MEMORY
            .lock()
            .expect("Unable to fetch Bills Lock")
            .clear();
    }
}
