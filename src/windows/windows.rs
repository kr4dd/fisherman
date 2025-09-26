use crate::windows::find_running_processes;

/// Detect VBOX environment processes 
pub fn detect_vbox_usage() {
    let target_processes = vec!["VBoxService.exe", "VBoxTray.exe"];
    let running = find_running_processes(target_processes);

    for process in running {
        println!("[*] Virtual environment process VBOX found: {}", process);
    }
}

/// Detect VMWARE environment processes
pub fn detect_vmware_usage() {
    let target_processes = vec!["VMwareService.exe",
     "VMwareTray.exe", "TPAutoConnSvc.exe", "VMtoolsd.exe", "VMwareuser.exe"];
    let running = find_running_processes(target_processes);

    for process in running {
        println!("[*] Virtual environment process VMWARE found: {}", process);
    }
} 