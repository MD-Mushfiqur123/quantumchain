//! Core types used throughout the blockchain

use serde::{Deserialize, Serialize};
use std::fmt;

/// Block hash (32 bytes)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BlockHash(pub [u8; 32]);

impl fmt::Display for BlockHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{}", hex::encode(self.0))
    }
}

/// Transaction hash (32 bytes)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TxHash(pub [u8; 32]);

impl fmt::Display for TxHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{}", hex::encode(self.0))
    }
}

/// Account address (20 bytes, Ethereum-compatible)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Address(pub [u8; 20]);

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{}", hex::encode(self.0))
    }
}

/// Balance amount (256-bit unsigned integer)
pub type Balance = u128;

/// Gas amount
pub type Gas = u64;

/// Block number
pub type BlockNumber = u64;

/// Timestamp (Unix timestamp in seconds)
pub type Timestamp = u64;

/// Nonce for transactions
pub type Nonce = u64;

/// Signature (64 bytes for Ed25519)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature(pub [u8; 64]);

/// Public key (32 bytes for Ed25519)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicKey(pub [u8; 32]);

/// Transaction structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// Sender address
    pub from: Address,
    /// Recipient address (None for contract creation)
    pub to: Option<Address>,
    /// Amount to transfer
    pub value: Balance,
    /// Transaction data (contract call or deployment code)
    pub data: Vec<u8>,
    /// Gas limit
    pub gas_limit: Gas,
    /// Gas price
    pub gas_price: Balance,
    /// Nonce
    pub nonce: Nonce,
    /// Signature
    pub signature: Signature,
}

/// Block header
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    /// Block number
    pub number: BlockNumber,
    /// Parent block hash
    pub parent_hash: BlockHash,
    /// Timestamp
    pub timestamp: Timestamp,
    /// State root
    pub state_root: BlockHash,
    /// Transactions root
    pub transactions_root: BlockHash,
    /// Receipts root
    pub receipts_root: BlockHash,
    /// Block proposer
    pub proposer: Address,
    /// Extra data
    pub extra_data: Vec<u8>,
}

/// Complete block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    /// Block header
    pub header: BlockHeader,
    /// Transactions in this block
    pub transactions: Vec<Transaction>,
}

impl Block {
    /// Calculate block hash
    pub fn hash(&self) -> BlockHash {
        let encoded = bincode::serialize(&self.header).unwrap();
        let hash = blake3::hash(&encoded);
        BlockHash(hash.into())
    }
}

/// Transaction receipt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Receipt {
    /// Transaction hash
    pub tx_hash: TxHash,
    /// Block number
    pub block_number: BlockNumber,
    /// Gas used
    pub gas_used: Gas,
    /// Success status
    pub success: bool,
    /// Logs emitted
    pub logs: Vec<Log>,
    /// Contract address (if contract creation)
    pub contract_address: Option<Address>,
}

/// Event log
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Log {
    /// Contract address that emitted the log
    pub address: Address,
    /// Topics (indexed parameters)
    pub topics: Vec<[u8; 32]>,
    /// Data (non-indexed parameters)
    pub data: Vec<u8>,
}

/// Account state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    /// Account balance
    pub balance: Balance,
    /// Transaction nonce
    pub nonce: Nonce,
    /// Contract code hash (None for EOA)
    pub code_hash: Option<BlockHash>,
    /// Storage root
    pub storage_root: BlockHash,
}

impl Default for Account {
    fn default() -> Self {
        Self {
            balance: 0,
            nonce: 0,
            code_hash: None,
            storage_root: BlockHash([0; 32]),
        }
    }
}

// Helper module for hex encoding
mod hex {
    pub fn encode(bytes: impl AsRef<[u8]>) -> String {
        bytes.as_ref().iter().map(|b| format!("{:02x}", b)).collect()
    }
}
