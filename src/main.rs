const MAX_BLOCK_SIZE: usize = 100;

pub type Time = usize;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Transaction {
    pub fee_price: usize,
    pub size: usize,
    pub broadcasted_at: Time,
}

pub trait TransactionBroadcaster {
    fn emit_at(&self, time: Time) -> Option<Transaction>;
    fn broadcast_at(&self, time: Time, pool: &mut TransactionPool) {
        if let Some(transaction) = self.emit_at(time) {
            println!("tx broadcast at {}", time);
            pool.append(transaction)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TransactionPool {
    transactions: Vec<Transaction>,
}

impl TransactionPool {
    pub fn new() -> Self {
        Self {
            transactions: vec![],
        }
    }
    pub fn append(&mut self, transaction: Transaction) {
        let index = match self
            .transactions
            .binary_search_by(|probe| probe.fee_price.cmp(&transaction.fee_price))
        {
            Ok(index) => index,
            Err(index) => index,
        };
        self.transactions.insert(index, transaction)
    }

    pub fn pop_transactions(&mut self, max_total_size: usize) -> Vec<Transaction> {
        let mut transactions = vec![];
        let mut total_size = 0;
        for transaction in self.transactions.iter().rev() {
            total_size += transaction.size;
            if total_size > max_total_size {
                break;
            }
            transactions.push(*transaction);
        }
        transactions
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Block {
    pub mined_at: Time,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(mined_at: Time, transactions: Vec<Transaction>) -> Result<Self, String> {
        if transactions.iter().fold(0, |acc, x| acc + x.size) < MAX_BLOCK_SIZE {
            return Err("total size exceed".into());
        }
        Ok(Self {
            mined_at,
            transactions,
        })
    }
}

pub trait BlockMiner {
    fn is_mine_at(&self, time: Time) -> bool;
    fn mine_at(&self, time: Time, pool: &mut TransactionPool) -> Option<Block> {
        if self.is_mine_at(time) {
            let transactions = pool.pop_transactions(MAX_BLOCK_SIZE);
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

pub struct UniformTransactionBroadCaster {
    pub interval: Time,
    pub fee_price: usize,
    pub tx_size: usize,
}

impl TransactionBroadcaster for UniformTransactionBroadCaster {
    fn emit_at(&self, time: Time) -> Option<Transaction> {
        if time % self.interval == 0 {
            Some(Transaction {
                broadcasted_at: time,
                fee_price: self.fee_price,
                size: self.tx_size,
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
    fn is_mine_at(&self, time: usize) -> bool {
        time % self.interval == 0
    }
}

pub fn run<T: TransactionBroadcaster, B: BlockMiner>(
    transaction_broad_caster: T,
    block_miner: B,
    end_time: Time,
) -> Vec<Block> {
    let mut pool = TransactionPool::new();
    let mut blocks = vec![];
    for time in 0..end_time {
        transaction_broad_caster.broadcast_at(time, &mut pool);
        if let Some(block) = block_miner.mine_at(time, &mut pool) {
            blocks.push(block)
        }
    }
    blocks
}

fn main() {
    let transaction_broad_caster = UniformTransactionBroadCaster {
        interval: 5,
        fee_price: 100,
        tx_size: 10,
    };

    let block_miner = UniformBlockMiner { interval: 50 };

    let blocks = run(transaction_broad_caster, block_miner, 1000);
    println!("result: {:?}", blocks);
}
