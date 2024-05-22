use crate::transaction::TransactionType;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

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
            accounts: HashMap::new(),
            blocks: Vec::new(),
            transaction_pool: Vec::new(),
        }
    }

    pub fn get_balance(&mut self, account_id: u32) -> Option<u32> {
        self.accounts.get(&account_id).copied()
    }

    pub fn add_transaction(&mut self, tx: TransactionType) {
        self.transaction_pool.push(tx);
    }

    pub fn mine_block(&mut self) -> () {
        let mut new_accounts = self.accounts.clone();
        let mut block_data: Vec<TransactionType> = Vec::new();

        for tx in &self.transaction_pool {
            if tx.validate(&new_accounts) {
                tx.execute(&mut new_accounts);
                block_data.push(*(tx));
            }
        }

        let seconds = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as u32;

        let block = Block {
            timestamp: seconds,
            data: block_data,
        };

        self.accounts = new_accounts;
        self.blocks.push(block);
    }
}
