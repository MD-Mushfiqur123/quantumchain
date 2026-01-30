//! QuantumChain Core Library
//!
//! This is the core implementation of the QuantumChain blockchain.
//! It includes consensus, networking, VM, storage, and all advanced features.

#![warn(missing_docs)]
#![warn(clippy::all)]

pub mod consensus;
pub mod network;
pub mod vm;
pub mod storage;
pub mod crypto;
pub mod types;
pub mod config;

// Advanced modules (Phase 2+)
pub mod bridge;
pub mod defi;
pub mod ai;
pub mod governance;
pub mod privacy;
pub mod rollups;
pub mod sharding;

/// Re-export commonly used types
pub mod prelude {
    pub use crate::consensus::*;
    pub use crate::network::*;
    pub use crate::vm::*;
    pub use crate::storage::*;
    pub use crate::types::*;
}

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Protocol version
pub const PROTOCOL_VERSION: u32 = 1;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}
