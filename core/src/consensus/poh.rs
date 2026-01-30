//! Proof of History implementation (Solana-inspired)

use super::*;
use sha3::{Digest, Sha3_256};
use std::sync::atomic::{AtomicU64, Ordering};

/// Proof of History - cryptographic clock
pub struct ProofOfHistory {
    /// Current hash in the sequence
    current_hash: parking_lot::Mutex<[u8; 32]>,
    
    /// Tick counter
    tick_count: AtomicU64,
}

impl ProofOfHistory {
    /// Create new PoH instance
    pub fn new() -> Self {
        Self {
            current_hash: parking_lot::Mutex::new([0; 32]),
            tick_count: AtomicU64::new(0),
        }
    }
    
    /// Generate next tick
    pub fn tick(&self) -> [u8; 32] {
        let mut hash = self.current_hash.lock();
        
        // Hash the previous hash
        let mut hasher = Sha3_256::new();
        hasher.update(&*hash);
        let result = hasher.finalize();
        
        *hash = result.into();
        self.tick_count.fetch_add(1, Ordering::SeqCst);
        
        *hash
    }
    
    /// Get current timestamp (tick count)
    pub async fn get_timestamp(&self) -> u64 {
        self.tick_count.load(Ordering::SeqCst)
    }
    
    /// Validate timestamp
    pub async fn validate_timestamp(&self, timestamp: u64) -> ConsensusResult<()> {
        let current = self.tick_count.load(Ordering::SeqCst);
        
        // Timestamp should be close to current tick
        if timestamp > current + 1000 {
            return Err(ConsensusError::InvalidBlock("Future timestamp".to_string()));
        }
        
        Ok(())
    }
    
    /// Get current hash
    pub fn get_current_hash(&self) -> [u8; 32] {
        *self.current_hash.lock()
    }
}

impl Default for ProofOfHistory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poh_ticking() {
        let poh = ProofOfHistory::new();
        
        let hash1 = poh.tick();
        let hash2 = poh.tick();
        
        // Hashes should be different
        assert_ne!(hash1, hash2);
        
        // Tick count should increase
        assert_eq!(poh.tick_count.load(Ordering::SeqCst), 2);
    }
}
