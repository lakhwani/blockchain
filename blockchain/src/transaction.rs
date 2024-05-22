use std::collections::HashMap;

pub trait Transaction {
    fn validate(&self, accounts: &HashMap<u32, u32>) -> bool;
    fn addToPool(&self, accounts: &mut HashMap<u32, u32>, txPool: &mut Vec<Transaction>) -> f64;
    fn execute(&self, accounts: &mut HashMap<u32, u32>) -> bool;
}

pub struct CreateAccount {
    pub account_id: u32,
    pub startingBalance: u32,
}

pub struct Transfer {
    pub fromAccount: u32,
    pub toAccount: u32,
    pub amount: u32,
}

impl Transaction for CreateAccount {
    fn validate(&self, accounts: &mut HashMap<u32, u32>) {
        if &self.startingBalance < 0 {
            println!("Starting balance does not exists:", self.accountId);
            return False;
        }
        if accounts.contains_key(&self.accountId) {
            println!("Account ID already exists:", self.accountId);
            return False;
        }
        return True;
    }

    fn addToPool(&self, accounts: &mut HashMap<u32, u32>, txPool: &mut Vec<Transaction>) {
        if validate(&self, accounts, txPool) {
            txPool.push(self);
        }
    }

    fn execute(&self, accounts: &mut HashMap<u32, u32>) {
        accounts.insert(&self.accountId, startingBalance);
    }
}

impl Transaction for Transfer {
    fn validate(&self, accounts: &mut HashMap<u32, u32>) {
        if !accounts.contains_key(&self.fromAccount) {
            println!("From Account does not exist:", self.fromAccount);
            return False;
        }
        if !accounts.contains_key(&self.toAccount) {
            println!("To Account does not exist:", self.fromAccount);
            return False;
        }
        let Some(fromBalance) = accounts.get(&self.fromAccount);
        if fromBalance <= amount {
            println!("User does not have sufficient balance.");
            return False;
        }
        return True;
    }

    fn addToPool(&self, accounts: &mut HashMap<u32, u32>, txPool: &mut Vec<Transaction>) {
        if validate(&self, accounts, txPool) {
            txPool.push(self)
        }
    }

    fn execute(&self, accounts: &mut HashMap<u32, u32>) {
        validate(&self, accounts);
        let from_balance = accounts[&self.from_account];
        let to_balance = accounts[&self.to_account];
        accounts.insert(&self.fromAccount, oldFromBalance - &self.amount);
        accounts.insert(&self.toAccount, oldToBalance + &self.amount)
    }
}
