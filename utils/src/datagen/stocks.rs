use rand::Rng;

use crate::json::{CompanyJSON, StockJSON};

use super::Generator;

pub struct StockGenerator {
    generator: Generator
}

impl StockGenerator {
    pub fn new() -> StockGenerator {
        StockGenerator {
            generator: Generator::new()
        }
    }
    pub fn create(&mut self, company: &CompanyJSON) -> Vec<StockJSON> {
        let stocks = self.generator.random.gen_range(1..1000); // 1 - 999 stocks
        let value = self.generator.random.gen_range(1..(1_000_000/stocks));
        (0..stocks).into_iter().map(|nth| {
            StockJSON {
                id: self.generator.next(),
                owner: company.performer,
                issued_by: company.clone(),
                share: value,
                bought_for: 0,
            }
        }).collect()
    }
}