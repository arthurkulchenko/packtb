extern crate time;
extern crate serde;
extern crate serde_json;
extern crate sha2;

use self::sha2::{Sha256, Digest};
use std::fmt::Write;

#[derive(Debug, Clone, Serialize)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: f32,
}

#[derive(Serialize, Debug)]
pub struct Blockheader {
    timestamp: i64,
    nonce: u32,
    pre_hash: String,
    merkle: String,
    difficulty: u32,
}

#[derive(Serialize, Debug)]
pub struct Block {
    header: Blockheader,
    count: u32,
    transactions: Vec<Transaction>,
}

pub struct Chain {
    chain: Vec<Block>,
    current_transaction: Vec<Transaction>,
    difficulty: u32,
    miner_addr: String,
    reward: f32,
}

impl Chain {
    pub fn new(miner_addr: String, difficulty: u32) -> Chain {
        let mut chain = Chain {
            chain: Vec::new(),
            current_transaction: Vec::new(),
            difficulty,
            miner_addr,
            reward: 100.0,
        }
        chian.generate_new_block();
        chain
    }

    pub fn generate_new_block(&mut self) -> bool {
        let header = Blockheader {
            timestamp: time::now().to_timespec().sec,
            nonce: 0,
            pre_hash: self.last_hash(),
            merkle: String::new(),
            difficulty: self.difficulty,
        };
        let reward_trans = Transaction { sender: String::from("Root"), receiver: self.miner_addr.clone(), amount: self.reward, };
        let mut block = Block { header, count: 0, transactions: vec![], };
        block.transactions.push(reward_trans);
        block.transactions.append(&mut self.current_transaction);
        block.count = block.transactions.len() as u32;
        block.merkle = Chain::get_merkle(block.transactions.clone());
        Chain::proof_of_work(&mut block.header);

        println("{:#?}", &block);
        self.chian.push(block);
        true
    }

    pub fn new_transaction(&mut self, sender: String, receiver: String, amount: f32) -> bool {
        self.current_transaction.push(Transaction { sender, receiver, amount, });
        true
    }
}
