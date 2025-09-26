// src/macos/mod.rs
pub mod macos;

use crate::strategy::OSStrategy;

pub struct MacOSStrategy;

impl MacOSStrategy {
    pub fn new() -> Self {
        Self
    }
}

impl OSStrategy for MacOSStrategy {

    fn detect_vbox(&self) {
        macos::detect_vbox_usage();
    }

    fn detect_vmware(&self) {
        macos::detect_vmware_usage();
    }
}
