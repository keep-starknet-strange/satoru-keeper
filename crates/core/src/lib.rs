//! The core crate contains the core types and traits for the Satoru Keeper service.

use log::info;
/// The Keeper struct is the main entry point for the Keeper service.
#[derive(Clone, Debug, Default)]
pub struct Keeper {}

impl Keeper {
    /// Execute a deposit.
    /// # Arguments
    /// * `deposit_key` - The deposit key.
    pub fn execute_deposit(&self, deposit_key: &str) {
        info!("running execute_deposit with key: {}", deposit_key);
    }
}
