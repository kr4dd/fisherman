// src/strategy.rs
pub trait OSStrategy {

    /// Virtual environment techniques detection
    fn detect_vbox(&self);
    fn detect_vmware(&self);
}