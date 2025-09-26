// src/linux/mod.rs
pub mod linux;

use crate::strategy::OSStrategy;

pub struct LinuxStrategy;

impl LinuxStrategy {
    pub fn new() -> Self {
        Self
    }
}

impl OSStrategy for LinuxStrategy {

    fn detect_vbox(&self) {
        linux::detect_vbox_usage()
    }

    fn detect_vmware(&self) {
        linux::detect_vmware_usage()
    }   

}
