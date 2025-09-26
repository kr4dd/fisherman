// src/lib.rs
pub mod strategy;
pub mod windows;
pub mod linux;
pub mod macos;

use std::env;
use strategy::OSStrategy;

pub struct OSContext {
    strategy: Box<dyn OSStrategy>,
    current_os: String,
}

impl OSContext {
    pub fn new() -> Self {
        let current_os = env::consts::OS.to_string();
        let strategy = Self::create_strategy_for_os(&current_os);
        
        Self {
            strategy,
            current_os,
        }
    }

    pub fn with_specific_os(os_name: &str) -> Self {
        let strategy = Self::create_strategy_for_os(os_name);
        
        Self {
            strategy,
            current_os: os_name.to_string(),
        }
    }

    fn create_strategy_for_os(os_name: &str) -> Box<dyn OSStrategy> {
        match os_name {
            "windows" => Box::new(windows::WindowsStrategy::new()),
            "linux" => Box::new(linux::LinuxStrategy::new()),
            "macos" => Box::new(macos::MacOSStrategy::new()),
            _ => {
                println!("Unknown OS: {}", os_name);
                std::process::exit(1);
            }
        }
    }

    pub fn switch_strategy(&mut self, os_name: &str) {
        self.strategy = Self::create_strategy_for_os(os_name);
        self.current_os = os_name.to_string();
        println!("Switched to {} strategy", os_name);
    }

    pub fn get_current_os(&self) -> &str {
        &self.current_os
    }

    // Delegate methods to the current strategy
    pub fn detect_vbox(&self) {
        self.strategy.detect_vbox();
    }

    pub fn detect_vmware(&self) {
        self.strategy.detect_vmware();
    }

}
