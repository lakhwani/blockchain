mod block;

use block::Block;
use std::collections::HashMap;

enum Blockchain {
  Empty,
  Elem(Block, Box<List>),
}

struct B {
  accounts: HashMap<String, u32>::new()
  transactionPool: Vec::new<Transaction>()
  blocks: Blockchain
}