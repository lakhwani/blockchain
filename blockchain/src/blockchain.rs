use std::collections::HashMap;

pub struct Block {
    timestamp: u32,
    data: Vec<Transaction>::new(),
}

pub enum List {
    Empty,
    Elem(Block, Box<List>),
}

impl List {
    fn new() -> Self {
        List::Empty
    }

    fn prepend(self, elem: Block) -> Self {
        List::Elem(elem, Box::new(self))
    }
}

pub struct Block {
    timestamp: u32,
    data: Vec<Transaction>::new(),
}

struct B {
    accounts: HashMap<String, u32>::new(),
    transactionPool: Vec<Transaction>::new(),
    blocks: List::new(),
}

impl B {
    fn clearTxPool() {
        self.transactionPool.clear()
    }

    fn getBalance(accountId: u32) -> u32 {
        let Some(value) = self.accounts.get(accountId);
        return value;
    }
}
