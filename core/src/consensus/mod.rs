//! Consensus module
//!
//! Implements hybrid PoS + aBFT + PoH consensus mechanism

use crate::types::*;
use crate::config::ConsensusConfig;
use async_trait::async_trait;
use std::sync::Arc;
use thiserror::Error;

pub mod pos;
pub mod abft;
pub mod poh;
pub mod dag;
pub mod validator;

/// Consensus errors
#[derive(Error, Debug)]
pub enum ConsensusError {
    #[error("Invalid block: {0}")]
    InvalidBlock(String),
    
    #[error("Invalid signature")]
    InvalidSignature,
    
    #[error("Insufficient stake")]
    InsufficientStake,
    
    #[error("Validator not found")]
    ValidatorNotFound,
    
    #[error("Consensus timeout")]
    Timeout,
}

/// Result type for consensus operations
pub type ConsensusResult<T> = Result<T, ConsensusError>;

/// Consensus engine trait
#[async_trait]
pub trait ConsensusEngine: Send + Sync {
    /// Propose a new block
    async fn propose_block(&self, transactions: Vec<Transaction>) -> ConsensusResult<Block>;
    
    /// Validate a block
    async fn validate_block(&self, block: &Block) -> ConsensusResult<()>;
    
    /// Finalize a block
    async fn finalize_block(&self, block: &Block) -> ConsensusResult<()>;
    
    /// Get current validators
    async fn get_validators(&self) -> ConsensusResult<Vec<Validator>>;
    
    /// Check if we are the current proposer
    async fn is_proposer(&self) -> ConsensusResult<bool>;
}

/// Validator information
#[derive(Debug, Clone)]
pub struct Validator {
    /// Validator address
    pub address: Address,
    
    /// Staked amount
    pub stake: Balance,
    
    /// Public key
    pub public_key: PublicKey,
    
    /// Reputation score
    pub reputation: u64,
    
    /// Is active
    pub is_active: bool,
}

/// Hybrid consensus implementation
pub struct HybridConsensus {
    config: Arc<ConsensusConfig>,
    pos: pos::ProofOfStake,
    abft: abft::AsyncBFT,
    poh: Option<poh::ProofOfHistory>,
    dag: Option<dag::DagStructure>,
}

impl HybridConsensus {
    /// Create new hybrid consensus engine
    pub fn new(config: ConsensusConfig) -> Self {
        let config = Arc::new(config);
        
        let pos = pos::ProofOfStake::new(config.clone());
        let abft = abft::AsyncBFT::new(config.clone());
        
        let poh = if config.enable_poh {
            Some(poh::ProofOfHistory::new())
        } else {
            None
        };
        
        let dag = if config.enable_dag {
            Some(dag::DagStructure::new())
        } else {
            None
        };
        
        Self {
            config,
            pos,
            abft,
            poh,
            dag,
        }
    }
}

#[async_trait]
impl ConsensusEngine for HybridConsensus {
    async fn propose_block(&self, transactions: Vec<Transaction>) -> ConsensusResult<Block> {
        // 1. Check if we are the proposer (PoS selection)
        let proposer = self.pos.select_proposer().await?;
        
        // 2. Add PoH timestamp if enabled
        let timestamp = if let Some(poh) = &self.poh {
            poh.get_timestamp().await
        } else {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        };
        
        // 3. Create block header
        let header = BlockHeader {
            number: 0, // TODO: Get from chain state
            parent_hash: BlockHash([0; 32]), // TODO: Get from chain state
            timestamp,
            state_root: BlockHash([0; 32]), // TODO: Calculate
            transactions_root: BlockHash([0; 32]), // TODO: Calculate
            receipts_root: BlockHash([0; 32]), // TODO: Calculate
            proposer,
            extra_data: vec![],
        };
        
        // 4. Create block
        let block = Block {
            header,
            transactions,
        };
        
        Ok(block)
    }
    
    async fn validate_block(&self, block: &Block) -> ConsensusResult<()> {
        // 1. Validate block structure
        if block.transactions.is_empty() {
            return Err(ConsensusError::InvalidBlock("Empty block".to_string()));
        }
        
        // 2. Validate proposer
        self.pos.validate_proposer(&block.header.proposer).await?;
        
        // 3. Validate with aBFT
        self.abft.validate(block).await?;
        
        // 4. Validate PoH if enabled
        if let Some(poh) = &self.poh {
            poh.validate_timestamp(block.header.timestamp).await?;
        }
        
        Ok(())
    }
    
    async fn finalize_block(&self, block: &Block) -> ConsensusResult<()> {
        // Use aBFT for instant finality
        self.abft.finalize(block).await?;
        Ok(())
    }
    
    async fn get_validators(&self) -> ConsensusResult<Vec<Validator>> {
        self.pos.get_validators().await
    }
    
    async fn is_proposer(&self) -> ConsensusResult<bool> {
        self.pos.is_proposer().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_consensus_creation() {
        let config = ConsensusConfig::default();
        let consensus = HybridConsensus::new(config);
        assert!(consensus.poh.is_some());
        assert!(consensus.dag.is_some());
    }
}
