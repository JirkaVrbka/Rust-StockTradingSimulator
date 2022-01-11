use rand::Rng;

use crate::json::{CompanyJSON, StockJSON};

use super::{Generator, Data};

pub struct StockGenerator;

impl StockGenerator {
    pub fn new() -> StockGenerator {
        StockGenerator { }
    }
    pub fn create(&mut self, generator: &mut Generator, data: &mut Data) {
        let count = data.next();
        let company = generator.choose(&mut data.companies);
        let stocks = generator.random.gen_range(1..1000); // 1 - 999 stocks
        let value = generator.random.gen_range(1..(1_000_000/stocks));
        for nth in 0..stocks {
            data.stocks.push_back(
            StockJSON {
                id: count + nth,
                owner: company.performer.clone(),
                issued_by: company.clone(),
                share: value,
                bought_for: 0,
            });
        }
    }
}