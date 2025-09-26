// src/windows/mod.rs
pub mod windows;

use crate::strategy::OSStrategy;

use sysinfo::{System};

pub struct WindowsStrategy;

impl WindowsStrategy {
    pub fn new() -> Self {
        Self
    }
}

impl OSStrategy for WindowsStrategy {

    fn detect_vbox(&self){
        windows::detect_vbox_usage();
    }   

    fn detect_vmware(&self) {
        windows::detect_vmware_usage();
    }

}


pub fn find_running_processes(process_names: Vec<&str>) -> Vec<String> {
    let mut system = System::new_all();
    system.refresh_all();

    let mut found_processes = Vec::new();

    for process in system.processes().values() {
        let process_name = process.name();
        for name_to_find in &process_names {
            if process_name.eq_ignore_ascii_case(name_to_find) {
                found_processes.push(process_name.to_string_lossy().to_string());
            }
        }
    }

    found_processes
}
