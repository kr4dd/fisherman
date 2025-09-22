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

    fn detect_vbox(&self) -> Result<Vec<String>, bool> {
        linux::detect_vbox_usage()
    }   

}
