use std::collections::HashMap;

use chrono::NaiveDateTime;
use rand::{Rng};

use crate::json::{StockJSON, StonkerJSON, HistoryJSON};

use super::{Generator, IndexVec};
use rand_distr::{Normal, Exp};


struct HistoryGenerator {
    generator: Generator,
    stonkers: IndexVec<StonkerJSON>,
    stocks: IndexVec<StockJSON>,
    spawned: HashMap<i32, Vec<HistoryJSON>> // stock id -> history
}

impl HistoryGenerator {
    fn new() -> HistoryGenerator {
        HistoryGenerator {
            generator: Generator::new(),
            stonkers: IndexVec::new(),
            stocks: IndexVec::new(),
            spawned: HashMap::new(),
        }
    }
    fn insert(&mut self, stonkers: Vec<StonkerJSON>, stocks: Vec<StockJSON>) {
        self.stonkers.extend(stonkers);
        self.stocks.extend(stocks);
    }
    fn create(&mut self) -> &HistoryJSON {
        let stonker = self.generator.choose(&mut self.stonkers);
        let stock = self.generator.choose(&mut self.stocks);
        let id = stock.id;
        let last_history = self.spawned.get_mut(&id);
        // we want to have > 0.95 percentage certainty
        // that stonker had enough money to buy the stock
        // Wolfram: CDF[NormalDistribution[0, 1], 2] - CDF[NormalDistribution[0, 1], -2] = 0.9545
        let normal = Normal::new(stock.bought_for as f64, (stonker.balance as f64) / 2.0).unwrap();
        let bought_for = self.generator.random.sample(normal) as i32;
        let now = chrono::offset::Local::now();
        let created_at = last_history.map_or_else(|| now.naive_local(), |history| {
            let difference = (now.timestamp() - history.last().unwrap().created_at.timestamp()) as f64;
            // we want to have > 0.95 percentage certainty that the
            // transaction happened in interval [last_transaction, last_transaction/2)
            // Wolfram: CDF[ExponentialDistribution[6], 0.5] = 0.950213
            let exponential = Exp::new(6.0).unwrap();
            let created_at = self.generator.random.sample(exponential) * difference;
            NaiveDateTime::from_timestamp(created_at.round() as i64, 0)
        });
        let history = HistoryJSON{
            owned_by: stonker.clone(),
            issued_by: stock.issued_by,
            bought_for: bought_for,
            created_at: created_at,
            sold_for
        };
        match last_history {
            None => { self.spawned.insert(id, vec![history]); },
            Some(vec) => vec.push(history),
        }
        &history
    }
}