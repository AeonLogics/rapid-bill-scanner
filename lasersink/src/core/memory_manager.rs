use crate::bills::LaserBillManager;
use crate::core::target::TargetSoftware;
use std::sync::Mutex;

static BILL_MEMORY: Mutex<LaserBillManager> = Mutex::new(LaserBillManager::init());
static TARGET_SOFTWARE_MEMORY: Mutex<TargetSoftware> = Mutex::new(TargetSoftware::init());
pub struct MemoryManager;

impl MemoryManager {
    pub fn init() -> MemoryManager {
        MemoryManager
    }
}
