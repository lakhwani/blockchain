use crate::transaction::TransactionType;
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;


pub struct Block {
    pub timestamp: u32,
    pub data: Vec<TransactionType>,
}

pub struct Blockchain {
    pub accounts: HashMap<u32, u32>,
    pub blocks: Vec<Block>,
    pub transaction_pool: Vec<TransactionType>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain { 
            accounts: HashMap<u32, u32>::new(), 
            blocks: Vec<Block>::new(), 
            transaction_pool: Vec<TransactionType>::new() 
        }
    }

    pub fn get_balance(&mut self, accountId: u32) -> Option<u32> {
        self.accounts.get(&account_id).copied()
    }

    pub fn add_transaction(&mut self, tx: TransactionType){
        self.transaction_pool.push(tx);
    }

    pub fn mine_block(&mut self) -> Block {
        let mut new_accounts = self.accounts.clone();
        let mut block_data = Vec<TransactionType>::new();

        for tx in &self.transaction_pool {
            if (tx.validate(&new_accounts)){
                tx.execute(&mut new_accounts);
                block_data.push(tx.clone());
            }
        }

        let now = SystemTime::now();
        let since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
        // Convert to seconds and ensure the value is within u32 range (0 to 4294967295)
        let seconds = since_epoch.as_secs() % u32::MAX as u64;

        let mut block = Block { timestamp: seconds, data: block_data };
        &self.blocks.accounts = new_accounts;
        self.blocks.push(block) 
    }
}
