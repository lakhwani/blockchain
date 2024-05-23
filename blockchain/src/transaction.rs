use std::collections::HashMap;

pub trait Transaction {
    fn validate(&self, accounts: &HashMap<u32, u32>) -> bool;
    fn execute(&self, accounts: &mut HashMap<u32, u32>);
}

#[derive(Copy, Clone)]
pub struct CreateAccount {
    pub account_id: u32,
    pub starting_balance: u32,
}

#[derive(Copy, Clone)]
pub struct Transfer {
    pub from_account: u32,
    pub to_account: u32,
    pub amount: u32,
}

impl Transaction for CreateAccount {
    fn validate(&self, accounts: &HashMap<u32, u32>) -> bool {
        if accounts.contains_key(&self.account_id) {
            println!("Account ID already exists: {}", self.account_id);
            return false;
        }
        true
    }

    fn execute(&self, accounts: &mut HashMap<u32, u32>) {
        accounts.insert(self.account_id, self.starting_balance);
    }
}

impl Transaction for Transfer {
    fn validate(&self, accounts: &HashMap<u32, u32>) -> bool {
        if !accounts.contains_key(&self.from_account) {
            println!("From Account does not exist: {}", self.from_account);
            return false;
        }
        if !accounts.contains_key(&self.to_account) {
            println!("To Account does not exist: {}", self.to_account);
            return false;
        }
        if accounts[&self.from_account] < self.amount {
            println!("Insufficient funds in account: {}", self.from_account);
            return false;
        }
        true
    }

    fn execute(&self, accounts: &mut HashMap<u32, u32>) {
        let from_balance = accounts[&self.from_account];
        let to_balance = accounts[&self.to_account];
        accounts.insert(self.from_account, from_balance - self.amount);
        accounts.insert(self.to_account, to_balance + self.amount);
    }
}

#[derive(Copy, Clone)]
pub enum TransactionType {
    CreateAccountTx(CreateAccount),
    TransferTx(Transfer),
}

impl TransactionType {
    pub fn validate(&self, accounts: &HashMap<u32, u32>) -> bool {
        match self {
            TransactionType::CreateAccountTx(tx) => tx.validate(accounts),
            TransactionType::TransferTx(tx) => tx.validate(accounts),
        }
    }

    pub fn execute(&self, accounts: &mut HashMap<u32, u32>) {
        match self {
            TransactionType::CreateAccountTx(tx) => tx.execute(accounts),
            TransactionType::TransferTx(tx) => tx.execute(accounts),
        }
    }
}
