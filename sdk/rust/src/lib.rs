//! QuantumChain Rust SDK

pub use quantum_core::types::*;

/// SDK client for interacting with QuantumChain
pub struct Client {
    rpc_url: String,
}

impl Client {
    /// Create new client
    pub fn new(rpc_url: impl Into<String>) -> Self {
        Self {
            rpc_url: rpc_url.into(),
        }
    }
    
    /// Get account balance
    pub async fn get_balance(&self, _address: &Address) -> Result<Balance, String> {
        // TODO: Implement RPC call
        Ok(0)
    }
    
    /// Send transaction
    pub async fn send_transaction(&self, _tx: Transaction) -> Result<TxHash, String> {
        // TODO: Implement transaction sending
        Ok(TxHash([0; 32]))
    }
}
