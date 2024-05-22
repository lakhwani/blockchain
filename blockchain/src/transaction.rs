use std::collections::HashMap;

pub trait Transaction {
    fn validate(&self, accounts: &HashMap<u32, u32>) -> bool;
    fn execute(&self, accounts: &mut HashMap<u32, u32>) -> bool;
}

pub struct CreateAccount {
    pub account_id: u32,
    pub starting_balance: u32,
}

pub struct Transfer {
    pub from_account: u32,
    pub to_account: u32,
    pub amount: u32,
}

impl Transaction for CreateAccount {
    fn validate(&self, accounts: &HashMap<u32, u32>) -> bool {
        if &self.startingBalance < 0 {
            println!(
                "Starting balance must be greater than 0: {}",
                self.starting_balance
            );
            return False;
        }
        if accounts.contains_key(&self.accountId) {
            println!("Account ID already exists: {}", self.account_id);
            return False;
        }
        return True;
    }

    fn execute(&self, accounts: &mut HashMap<u32, u32>) {
        accounts.insert(self.account_id, self.starting_balance);
    }
}

impl Transaction for Transfer {
    fn validate(&self, accounts: &mut HashMap<u32, u32>) {
        if !accounts.contains_key(&self.from_account) {
            println!("From Account does not exist: {}", self.from_account);
            return False;
        }
        if !accounts.contains_key(&self.to_account) {
            println!("To Account does not exist: {}", self.to_account);
            return False;
        }
        let cur_from_balance = accounts[&self.from_account];
        if cur_from_balance <= self.amount {
            println!(
                "User does not have sufficient balance in account: {}",
                self.from_account
            );
            return False;
        }
        return True;
    }

    fn execute(&self, accounts: &mut HashMap<u32, u32>) {
        let from_balance = accounts[&self.from_account];
        let to_balance = accounts[&self.to_account];
        accounts.insert(&self.fromAccount, oldFromBalance - &self.amount);
        accounts.insert(&self.toAccount, oldToBalance + &self.amount)
    }
}

pub enum Transactions {
    CreateAccount(CreateAccount),
    Transfer(Transfer),
}

impl Transactions {
    pub fn validate(&self, accounts: &HashMap<u32, u32>) -> bool {
        match self {
            Transactions::CreateAccount(tx) => tx.validate(accounts),
            Transactions::Transfer(tx) => tx.validate(accounts),
        }
    }

    pub fn execute(&self, accounts: &mut HashMap<u32, u32>) {
        match self {
            Transactions::CreateAccount(tx) => tx.execute(accounts),
            Transactions::Transfer(tx) => tx.execute(accounts),
        }
    }
}
