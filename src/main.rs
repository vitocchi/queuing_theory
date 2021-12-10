mod block;
mod report;
mod transaction;

pub use crate::block::*;
pub use crate::report::*;
pub use crate::transaction::*;
use statrs::distribution::{Exp, Normal};

pub type Time = usize;

fn main() {
    let transaction_broad_caster = DistributionTransactionBroadCaster::new(
        Exp::new(1.0 / 10_000.0).unwrap(),
        Normal::new(100_000.0, 10.0).unwrap(),
    );
    let block_miner = UniformBlockMiner { interval: 600_000 };

    let network = run(transaction_broad_caster, block_miner, 60_000 * 60 * 24);
    let report = TransactionReport::build(network);
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    report.write_csv(&format!("result/{}.csv", now)).unwrap();
}

pub fn run<T: TransactionBroadcaster, B: BlockMiner>(
    mut transaction_broad_caster: T,
    block_miner: B,
    end_time: Time,
) -> Network {
    let mut network = Network {
        pool: TransactionPool::new(),
        blocks: vec![],
    };
    for time in 0..end_time {
        transaction_broad_caster.broadcasted_at(time, &mut network.pool);
        if let Some(block) = block_miner.mine_at(time, &mut network.pool) {
            network.blocks.push(block)
        }
    }
    network
}
