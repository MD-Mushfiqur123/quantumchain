//! Configuration for the blockchain

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Global blockchain configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainConfig {
    /// Chain ID (for replay protection)
    pub chain_id: u64,
    
    /// Consensus configuration
    pub consensus: ConsensusConfig,
    
    /// Network configuration
    pub network: NetworkConfig,
    
    /// VM configuration
    pub vm: VmConfig,
    
    /// Storage configuration
    pub storage: StorageConfig,
}

impl Default for ChainConfig {
    fn default() -> Self {
        Self {
            chain_id: 1,
            consensus: ConsensusConfig::default(),
            network: NetworkConfig::default(),
            vm: VmConfig::default(),
            storage: StorageConfig::default(),
        }
    }
}

/// Consensus mechanism configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusConfig {
    /// Target block time in seconds
    pub block_time: Duration,
    
    /// Epoch length in blocks
    pub epoch_length: u64,
    
    /// Minimum stake required to be a validator
    pub min_validator_stake: u128,
    
    /// Maximum number of validators
    pub max_validators: usize,
    
    /// Slashing percentage for double signing
    pub slashing_percentage: u8,
    
    /// Enable Proof of History
    pub enable_poh: bool,
    
    /// Enable DAG structure
    pub enable_dag: bool,
}

impl Default for ConsensusConfig {
    fn default() -> Self {
        Self {
            block_time: Duration::from_secs(2),
            epoch_length: 100,
            min_validator_stake: 32_000_000_000_000_000_000, // 32 tokens
            max_validators: 1000,
            slashing_percentage: 5,
            enable_poh: true,
            enable_dag: true,
        }
    }
}

/// Network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// P2P listen address
    pub listen_addr: String,
    
    /// Bootstrap nodes
    pub bootstrap_nodes: Vec<String>,
    
    /// Maximum number of peers
    pub max_peers: usize,
    
    /// Enable QUIC protocol
    pub enable_quic: bool,
    
    /// Enable Tor/I2P
    pub enable_privacy_network: bool,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            listen_addr: "/ip4/0.0.0.0/tcp/30333".to_string(),
            bootstrap_nodes: vec![],
            max_peers: 50,
            enable_quic: true,
            enable_privacy_network: false,
        }
    }
}

/// Virtual machine configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmConfig {
    /// Enable WASM VM
    pub enable_wasm: bool,
    
    /// Enable EVM compatibility
    pub enable_evm: bool,
    
    /// Enable Move VM
    pub enable_move: bool,
    
    /// Enable parallel execution
    pub enable_parallel_execution: bool,
    
    /// Number of execution threads
    pub execution_threads: usize,
    
    /// Maximum gas per block
    pub max_gas_per_block: u64,
}

impl Default for VmConfig {
    fn default() -> Self {
        Self {
            enable_wasm: true,
            enable_evm: true,
            enable_move: false,
            enable_parallel_execution: true,
            execution_threads: num_cpus::get(),
            max_gas_per_block: 30_000_000,
        }
    }
}

/// Storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// Database path
    pub db_path: String,
    
    /// Enable state pruning
    pub enable_pruning: bool,
    
    /// Pruning history (blocks to keep)
    pub pruning_history: u64,
    
    /// Enable state snapshots
    pub enable_snapshots: bool,
    
    /// Snapshot interval (blocks)
    pub snapshot_interval: u64,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            db_path: "./data".to_string(),
            enable_pruning: true,
            pruning_history: 256,
            enable_snapshots: true,
            snapshot_interval: 1000,
        }
    }
}

// Helper to get CPU count
mod num_cpus {
    pub fn get() -> usize {
        std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4)
    }
}
