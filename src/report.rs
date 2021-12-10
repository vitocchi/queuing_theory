use super::*;
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Debug)]
pub struct TransactionReport {
    rows: Vec<TransactionReportRow>,
}
impl TransactionReport {
    pub fn build(network: Network) -> Self {
        let mut report = Self { rows: vec![] };
        for tx in network.pool.txs() {
            report.rows.push(TransactionReportRow {
                broadcasted_at: tx.broadcasted_at,
                fee_price: tx.fee_price,
                mined_at: None,
            })
        }
        for block in network.blocks {
            for tx in block.transactions {
                report.rows.push(TransactionReportRow {
                    broadcasted_at: tx.broadcasted_at,
                    fee_price: tx.fee_price,
                    mined_at: Some(block.mined_at),
                })
            }
        }
        report
    }

    pub fn write_csv(&self, path_to_file: &str) -> Result<(), String> {
        let mut wtr =
            csv::Writer::from_writer(File::create(path_to_file).map_err(|e| e.to_string())?);
        for row in self.rows.iter() {
            wtr.serialize(row.clone()).map_err(|e| e.to_string())?;
        }
        wtr.flush().map_err(|e| e.to_string())?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransactionReportRow {
    pub broadcasted_at: Time,
    pub fee_price: usize,
    pub mined_at: Option<Time>,
}
