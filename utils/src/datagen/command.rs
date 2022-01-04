use std::collections::HashMap;

use chrono::{NaiveDateTime, NaiveDate};
use rand::{Rng, prelude::SliceRandom};
use strum::IntoEnumIterator;

use crate::json::{CommandJSON, CommandTypesJSON, StonkerJSON, StockJSON};

use super::{Generator, IndexVec};

pub struct CommandGenerator {
    generator: Generator,
    spawned: HashMap<i32, Vec<CommandJSON>>,
}

impl CommandGenerator {
    pub fn new() -> CommandGenerator {
        CommandGenerator {
            generator: Generator::new(),
            spawned: HashMap::new(),
        }
    }
    fn get_beginning(&self) -> NaiveDateTime {
        NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11)
    }
    pub fn create(&mut self, stonkers: &mut IndexVec<StonkerJSON>, stock: &StockJSON) -> &CommandJSON {
        let stonker = self.generator.choose(stonkers).clone();
        let kind = CommandTypesJSON::iter().collect::<Vec<CommandTypesJSON>>()
            .choose(&mut self.generator.random).unwrap().clone();
        let id = stock.issued_by.id;
        let last_offer = self.spawned.get(&id);
        let created_at = match last_offer {
            None => self.get_beginning(),
            Some(command) => self.generator.random_date(command.last().unwrap().created_at),
        };
        let last_offer = self.spawned.get_mut(&id);
        let wealth = stonker.balance;
        let command = CommandJSON {
            id: self.generator.next(),
            stonker: stonker,
            kind: kind,
            created_at: created_at,
            company: stock.issued_by.clone(),
            threshold: self.generator.random_price(stock.bought_for, wealth),
            share: self.generator.random.gen_range(10_000..100_000)
        };
        match last_offer {
            None => { self.spawned.insert(id, vec![command]); },
            Some(vec) => { vec.push(command); },
        };
        self.spawned.get(&id).unwrap().last().unwrap()
    }
}