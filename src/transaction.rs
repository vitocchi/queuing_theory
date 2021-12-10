use super::*;
use rand::{distributions::Distribution, rngs::ThreadRng, thread_rng};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Transaction {
    pub fee_price: usize,
    pub broadcasted_at: Time,
}

pub trait TransactionBroadcaster {
    fn emit_at(&mut self, time: Time) -> Vec<Transaction>;
    fn broadcasted_at(&mut self, time: Time, pool: &mut TransactionPool) {
        for tx in self.emit_at(time) {
            println!("tx broad casted at {}", time);
            pool.append(tx)
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

    pub fn pop_transactions(&mut self, max: usize) -> Vec<Transaction> {
        let num = std::cmp::min(self.transactions.len(), max);
        self.transactions.drain(..num).collect()
    }

    pub fn txs(&self) -> &[Transaction] {
        &self.transactions
    }
}

pub struct UniformTransactionBroadCaster {
    pub interval: Time,
    pub fee_price: usize,
}

impl TransactionBroadcaster for UniformTransactionBroadCaster {
    fn emit_at(&mut self, time: Time) -> Vec<Transaction> {
        if time % self.interval == 0 {
            vec![Transaction {
                broadcasted_at: time,
                fee_price: self.fee_price,
            }]
        } else {
            vec![]
        }
    }
}

pub struct DistributionTransactionBroadCaster<E: Distribution<f64>, P: Distribution<f64>> {
    emit_distribution: E,
    next_emit_at: Time,
    price_distribution: P,
    rng: ThreadRng,
}

impl<E: Distribution<f64>, P: Distribution<f64>> DistributionTransactionBroadCaster<E, P> {
    pub fn new(emit_distribution: E, price_distribution: P) -> Self {
        let mut rng = thread_rng();
        let next_emit_at = emit_distribution.sample(&mut rng) as Time;

        Self {
            emit_distribution,
            next_emit_at,
            price_distribution,
            rng,
        }
    }
}

impl<E: Distribution<f64>, P: Distribution<f64>> TransactionBroadcaster
    for DistributionTransactionBroadCaster<E, P>
{
    fn emit_at(&mut self, time: Time) -> Vec<Transaction> {
        let mut txs = vec![];
        while time == self.next_emit_at {
            self.next_emit_at += self.emit_distribution.sample(&mut self.rng) as Time;
            txs.push(Transaction {
                fee_price: self.price_distribution.sample(&mut self.rng) as usize,
                broadcasted_at: time,
            });
        }
        txs
    }
}
