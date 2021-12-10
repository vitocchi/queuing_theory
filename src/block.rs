use super::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Network {
    pub pool: TransactionPool,
    pub blocks: Vec<Block>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Block {
    pub mined_at: Time,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(mined_at: Time, transactions: Vec<Transaction>) -> Result<Self, String> {
        Ok(Self {
            mined_at,
            transactions,
        })
    }
}

pub trait BlockMiner {
    fn is_mine_at(&self, time: Time) -> Option<usize>;
    fn mine_at(&self, time: Time, pool: &mut TransactionPool) -> Option<Block> {
        if let Some(max) = self.is_mine_at(time) {
            let transactions = pool.pop_transactions(max);
            println!("block mine at {}", time);
            Some(Block {
                mined_at: time,
                transactions,
            })
        } else {
            None
        }
    }
}

pub struct UniformBlockMiner {
    pub interval: Time,
}

impl BlockMiner for UniformBlockMiner {
    fn is_mine_at(&self, time: usize) -> Option<usize> {
        if time % self.interval == 0 {
            Some(50)
        } else {
            None
        }
    }
}
