// src/windows/mod.rs
pub mod windows;

use crate::strategy::OSStrategy;

pub struct WindowsStrategy;

impl WindowsStrategy {
    pub fn new() -> Self {
        Self
    }
}

impl OSStrategy for WindowsStrategy {

    fn detect_vbox(&self) -> Result<Vec<String>, bool> {
        windows::detect_vbox_usage()
    }   

}
