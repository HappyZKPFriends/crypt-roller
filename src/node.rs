pub mod history;
pub mod mempool;
pub mod state;

use history::History;
use mempool::Mempool;
use state::RollupState;
use state::TransactionExecutionError;

#[derive(Debug)]
pub enum NodeError {
    Transaction(TransactionExecutionError),
}

#[derive(Debug)]
pub struct Node {
    pub mempool: Mempool,
    pub history: History,
    pub state: RollupState,
}

impl Node {
    pub fn new() -> Self {
        Self {
            mempool: Mempool::new(),
            history: History::new(),
            state: RollupState::new(),
        }
    }

    pub fn start() -> Result<Self, NodeError> {
        Ok(Node::new())
    }

    fn apply_history(&mut self) -> Result<(), NodeError> {
        debug_assert!(*self.state.batch_count() <= self.history.len());

        let new_batches = self.history.len() - self.state.batch_count();
        if new_batches == 0 {
            println!("Node state already up to date with history.");
            return Ok(());
        }

        println!(
            "Applying {} transaction batches to rollup state:",
            new_batches
        );
        for batch in self.history.batches()[*self.state.batch_count()..self.history.len()].iter() {
            println!("    -> Applying {} transactions.", batch.len());
            self.state
                .apply_batch(batch.transactions())
                .map_err(NodeError::Transaction)?;
            for transaction in batch.transactions().iter() {
                self.mempool.drop_transaction(transaction);
            }
        }

        Ok(())
    }
}

impl Default for Node {
    fn default() -> Self {
        Self::new()
    }
}