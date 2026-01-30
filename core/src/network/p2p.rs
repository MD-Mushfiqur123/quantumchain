//! P2P networking layer

use crate::types::*;

/// P2P network manager
pub struct P2PNetwork {
    // TODO: Implement with libp2p
}

impl P2PNetwork {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for P2PNetwork {
    fn default() -> Self {
        Self::new()
    }
}
