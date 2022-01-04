use std::collections::HashMap;

use chrono::{NaiveDateTime, NaiveDate};
use rand::{Rng, prelude::SliceRandom};
use strum::IntoEnumIterator;

use crate::json::{StockJSON, StonkerJSON, HistoryJSON, CommandJSON, CommandTypesJSON};

use super::{Generator, IndexVec};
use rand_distr::{Normal, Exp};


// TODO: split to two separate generators that share stonkers and stocks
pub struct TradesGenerator {
    history_generator: Generator,
    offers_generator: Generator,
    stonkers: IndexVec<StonkerJSON>,
    stocks: IndexVec<StockJSON>,
    spawned_history: IndexVec<HistoryJSON>
    spawned_offers: HashMap<i32, Vec<CommandJSON>>
}

impl TradesGenerator {
    pub fn new() -> TradesGenerator {
        TradesGenerator {
            history_generator: Generator::new(),
            offers_generator: Generator::new(),
            stonkers: IndexVec::new(),
            stocks: IndexVec::new(),
            spawned_history: IndexVec::new(),
            spawned_offers: HashMap::new(),
        }
    }
    pub fn insert(&mut self, stonkers: Vec<StonkerJSON>, stocks: Vec<StockJSON>) {
        self.stonkers.extend(stonkers);
        self.stocks.extend(stocks);
    }
    // we want to have > 0.95 percentage certainty that the
    // transaction happened in interval [last_transaction, last_transaction/2)
    // Wolfram: CDF[ExponentialDistribution[6], 0.5] = 0.950213
    fn random_date(&self, last: NaiveDateTime) -> NaiveDateTime {
        let now = chrono::offset::Local::now();
        let difference = (now.timestamp() - last.timestamp()) as f64;
        let exponential = Exp::new(6.0).unwrap();
        let created_at = self.history_generator.random.sample(exponential) * difference;
        NaiveDateTime::from_timestamp(created_at.round() as i64, 0)
    }
    fn random_price(&self, last: i32, wealth: i32) -> i32 {
        // we want to have > 0.95 percentage certainty
        // that stonker had enough money to buy the stock
        // Wolfram: CDF[NormalDistribution[0, 1], 2] - CDF[NormalDistribution[0, 1], -2] = 0.9545
        let normal = Normal::new(last as f64, (wealth as f64) / 2.0).unwrap();
        self.offers_generator.random.sample(normal).round() as i32
    }
    fn get_beginning(&self) -> NaiveDateTime {
        NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11)
    }
    pub fn create_offer(&mut self) -> &CommandJSON {
        let stonker = self.offers_generator.choose(&mut self.stonkers).clone();
        let stock = self.offers_generator.choose(&mut self.stocks).clone();
        let kind = CommandTypesJSON::iter().collect::<Vec<CommandTypesJSON>>()
            .choose(&mut self.offers_generator.random).unwrap();
        let id = stock.issued_by.id;
        let last_offer = self.spawned_offers.get_mut(&id);
        let created_at = last_offer.map_or_else(|| self.get_beginning(), |command|
            self.random_date(command.last().unwrap().created_at));
        let command = CommandJSON {
            id: self.offers_generator.next(),
            stonker: stonker,
            kind: kind,
            created_at: created_at,
            company: stock.issued_by,
            threshold: self.random_price(stock.bought_for, stonker.balance),
            share: self.offers_generator.random.gen_range(10_000..100_000)
        };
        match last_offer {
            None => { self.spawned_offers.insert(id, vec![command]); },
            Some(vec) => vec.push(command),
        }
        &command
    }
    pub fn create_history(&mut self, sell_offer: &CommandJSON) -> &HistoryJSON {
        let id = self.history_generator.next();
        let created_at = self.random_date(sell_offer.created_at);
        let buyer = self.history_generator.choose(&mut self.stonkers).clone();
        let history = HistoryJSON{
            id: self.history_generator.next(),
            owned_by: sell_offer.stonker.clone(),
            issued_by: sell_offer.company.clone(),
            bought_for: sell_offer.threshold,
            created_at: created_at,
            sold_for: sell_offer.threshold,
        };
        self.spawned_history.push_back(history)
    }
}