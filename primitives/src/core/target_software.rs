use crate::core::KeyAction;
use gpui::{App, Global};
use std::fmt::Display;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum TargetSoftware {
    #[default]
    Nadra,
}

impl TargetSoftware {
    pub fn init(cx: &mut App) {
        let target = TargetSoftware::default();
        cx.set_global(target);
    }
    pub fn build_execute(&self, reference: &str, contact: &str) -> Vec<KeyAction> {
        match self {
            TargetSoftware::Nadra => vec![
                KeyAction::Text(reference.to_string()),
                KeyAction::Tab,
                KeyAction::Tab,
                KeyAction::Tab,
                KeyAction::Text(contact.to_string()),
                KeyAction::Tab,
                KeyAction::Space,
                KeyAction::Tab,
                KeyAction::Tab,
                KeyAction::Space,
            ],
        }
    }
}

impl Global for TargetSoftware {}

impl Display for TargetSoftware {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TargetSoftware::Nadra => write!(f, "Nadra"),
        }
    }
}
