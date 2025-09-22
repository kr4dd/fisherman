// src/main.rs
pub mod operations;

fn main() {
    println!("=== fisherman ===");
    operations::run_os_specific_operations();
}