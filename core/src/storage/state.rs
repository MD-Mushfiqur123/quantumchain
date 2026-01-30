//! State management with Verkle trees

use crate::types::*;

/// State manager
pub struct StateManager {
    // TODO: Implement Verkle trees
}

impl StateManager {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn get_account(&self, _address: &Address) -> Option<Account> {
        None
    }
    
    pub fn set_account(&mut self, _address: Address, _account: Account) {
        // TODO
    }
}

impl Default for StateManager {
    fn default() -> Self {
        Self::new()
    }
}
