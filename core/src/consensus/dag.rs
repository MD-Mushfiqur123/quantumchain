//! DAG (Directed Acyclic Graph) structure for parallel blocks

use super::*;
use std::collections::{HashMap, HashSet};

/// DAG-based block structure (Kaspa/Phantom inspired)
pub struct DagStructure {
    /// All blocks in the DAG
    blocks: parking_lot::RwLock<HashMap<BlockHash, Block>>,
    
    /// Parent relationships
    parents: parking_lot::RwLock<HashMap<BlockHash, Vec<BlockHash>>>,
    
    /// Children relationships
    children: parking_lot::RwLock<HashMap<BlockHash, Vec<BlockHash>>>,
}

impl DagStructure {
    /// Create new DAG structure
    pub fn new() -> Self {
        Self {
            blocks: parking_lot::RwLock::new(HashMap::new()),
            parents: parking_lot::RwLock::new(HashMap::new()),
            children: parking_lot::RwLock::new(HashMap::new()),
        }
    }
    
    /// Add block to DAG
    pub fn add_block(&self, block: Block) -> ConsensusResult<()> {
        let hash = block.hash();
        
        let mut blocks = self.blocks.write();
        let mut parents = self.parents.write();
        let mut children = self.children.write();
        
        // Add block
        blocks.insert(hash, block.clone());
        
        // Add parent relationship
        let parent_hash = block.header.parent_hash;
        parents.insert(hash, vec![parent_hash]);
        
        // Add child relationship
        children.entry(parent_hash)
            .or_insert_with(Vec::new)
            .push(hash);
        
        Ok(())
    }
    
    /// Get block by hash
    pub fn get_block(&self, hash: &BlockHash) -> Option<Block> {
        self.blocks.read().get(hash).cloned()
    }
    
    /// Get all tips (blocks with no children)
    pub fn get_tips(&self) -> Vec<BlockHash> {
        let blocks = self.blocks.read();
        let children = self.children.read();
        
        blocks.keys()
            .filter(|hash| !children.contains_key(hash))
            .copied()
            .collect()
    }
    
    /// Topological sort for ordering
    pub fn topological_sort(&self) -> Vec<BlockHash> {
        let blocks = self.blocks.read();
        let parents = self.parents.read();
        
        let mut sorted = Vec::new();
        let mut visited = HashSet::new();
        
        // Simple DFS-based topological sort
        for hash in blocks.keys() {
            if !visited.contains(hash) {
                self.dfs_visit(*hash, &parents, &mut visited, &mut sorted);
            }
        }
        
        sorted.reverse();
        sorted
    }
    
    fn dfs_visit(
        &self,
        hash: BlockHash,
        parents: &HashMap<BlockHash, Vec<BlockHash>>,
        visited: &mut HashSet<BlockHash>,
        sorted: &mut Vec<BlockHash>,
    ) {
        visited.insert(hash);
        
        if let Some(parent_list) = parents.get(&hash) {
            for parent in parent_list {
                if !visited.contains(parent) {
                    self.dfs_visit(*parent, parents, visited, sorted);
                }
            }
        }
        
        sorted.push(hash);
    }
}

impl Default for DagStructure {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dag_add_block() {
        let dag = DagStructure::new();
        
        let block = Block {
            header: BlockHeader {
                number: 1,
                parent_hash: BlockHash([0; 32]),
                timestamp: 1000,
                state_root: BlockHash([0; 32]),
                transactions_root: BlockHash([0; 32]),
                receipts_root: BlockHash([0; 32]),
                proposer: Address([0; 20]),
                extra_data: vec![],
            },
            transactions: vec![],
        };
        
        assert!(dag.add_block(block).is_ok());
    }
}
