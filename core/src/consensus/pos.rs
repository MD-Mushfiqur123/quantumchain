//! Proof of Stake implementation

use super::*;
use std::collections::HashMap;
use parking_lot::RwLock;

/// Proof of Stake engine
pub struct ProofOfStake {
    config: Arc<ConsensusConfig>,
    validators: Arc<RwLock<HashMap<Address, Validator>>>,
    current_epoch: Arc<RwLock<u64>>,
}

impl ProofOfStake {
    /// Create new PoS engine
    pub fn new(config: Arc<ConsensusConfig>) -> Self {
        Self {
            config,
            validators: Arc::new(RwLock::new(HashMap::new())),
            current_epoch: Arc::new(RwLock::new(0)),
        }
    }
    
    /// Select block proposer using VRF (Verifiable Random Function)
    pub async fn select_proposer(&self) -> ConsensusResult<Address> {
        let validators = self.validators.read();
        
        if validators.is_empty() {
            return Err(ConsensusError::ValidatorNotFound);
        }
        
        // Simple weighted random selection based on stake
        // TODO: Implement proper VRF (Algorand-style)
        let total_stake: u128 = validators.values()
            .filter(|v| v.is_active)
            .map(|v| v.stake)
            .sum();
        
        if total_stake == 0 {
            return Err(ConsensusError::InsufficientStake);
        }
        
        // For now, return first active validator
        // TODO: Implement cryptographic sortition
        let proposer = validators.values()
            .find(|v| v.is_active)
            .ok_or(ConsensusError::ValidatorNotFound)?;
        
        Ok(proposer.address)
    }
    
    /// Validate that address is an active validator
    pub async fn validate_proposer(&self, address: &Address) -> ConsensusResult<()> {
        let validators = self.validators.read();
        
        let validator = validators.get(address)
            .ok_or(ConsensusError::ValidatorNotFound)?;
        
        if !validator.is_active {
            return Err(ConsensusError::InvalidBlock("Inactive validator".to_string()));
        }
        
        if validator.stake < self.config.min_validator_stake {
            return Err(ConsensusError::InsufficientStake);
        }
        
        Ok(())
    }
    
    /// Get all validators
    pub async fn get_validators(&self) -> ConsensusResult<Vec<Validator>> {
        let validators = self.validators.read();
        Ok(validators.values().cloned().collect())
    }
    
    /// Check if current node is proposer
    pub async fn is_proposer(&self) -> ConsensusResult<bool> {
        // TODO: Implement based on node's own address
        Ok(false)
    }
    
    /// Add a new validator
    pub fn add_validator(&self, validator: Validator) -> ConsensusResult<()> {
        let mut validators = self.validators.write();
        
        if validator.stake < self.config.min_validator_stake {
            return Err(ConsensusError::InsufficientStake);
        }
        
        if validators.len() >= self.config.max_validators {
            return Err(ConsensusError::InvalidBlock("Max validators reached".to_string()));
        }
        
        validators.insert(validator.address, validator);
        Ok(())
    }
    
    /// Remove a validator
    pub fn remove_validator(&self, address: &Address) -> ConsensusResult<()> {
        let mut validators = self.validators.write();
        validators.remove(address)
            .ok_or(ConsensusError::ValidatorNotFound)?;
        Ok(())
    }
    
    /// Slash a validator for misbehavior
    pub fn slash_validator(&self, address: &Address) -> ConsensusResult<()> {
        let mut validators = self.validators.write();
        
        let validator = validators.get_mut(address)
            .ok_or(ConsensusError::ValidatorNotFound)?;
        
        // Reduce stake by slashing percentage
        let slash_amount = validator.stake * self.config.slashing_percentage as u128 / 100;
        validator.stake = validator.stake.saturating_sub(slash_amount);
        
        // Deactivate if stake too low
        if validator.stake < self.config.min_validator_stake {
            validator.is_active = false;
        }
        
        Ok(())
    }
    
    /// Advance to next epoch
    pub fn next_epoch(&self) {
        let mut epoch = self.current_epoch.write();
        *epoch += 1;
        
        // TODO: Recalculate validator set
        // TODO: Distribute rewards
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_validator() {
        let config = Arc::new(ConsensusConfig::default());
        let pos = ProofOfStake::new(config);
        
        let validator = Validator {
            address: Address([1; 20]),
            stake: 32_000_000_000_000_000_000,
            public_key: PublicKey([2; 32]),
            reputation: 100,
            is_active: true,
        };
        
        assert!(pos.add_validator(validator).is_ok());
    }
    
    #[test]
    fn test_insufficient_stake() {
        let config = Arc::new(ConsensusConfig::default());
        let pos = ProofOfStake::new(config);
        
        let validator = Validator {
            address: Address([1; 20]),
            stake: 1_000, // Too low
            public_key: PublicKey([2; 32]),
            reputation: 100,
            is_active: true,
        };
        
        assert!(pos.add_validator(validator).is_err());
    }
}
