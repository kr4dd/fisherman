// src/operations.rs
use fisherman::OSContext;

pub fn run_os_specific_operations() {
    let context = OSContext::new();
    println!("=== {} - Operations ===\n", context.get_current_os());

    println!("[-] Checking virtualization");
    println!("VBOX Info: {:?}", context.detect_vbox());
}
