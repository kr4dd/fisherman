// src/strategy.rs
pub trait OSStrategy {
    fn detect_vbox(&self) -> Result<Vec<String>, bool>;
}