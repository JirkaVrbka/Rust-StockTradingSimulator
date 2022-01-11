use crate::json::{HistoryJSON, CommandJSON, StonkerJSON};

use super::{Generator, IndexVec, Data};

pub struct HistoryGenerator {
    generator: Generator,
}

impl HistoryGenerator {
    pub fn new() -> HistoryGenerator {
        HistoryGenerator {
            generator: Generator::new(),
        }
    }
    pub fn create_history(&mut self, data: &mut Data) {
        let id = data.next();
        let sell_offer = self.generator.choose(&mut data.commands);
        let buyer = self.generator.choose(&mut data.stonkers).clone();
        let created_at = self.generator.random_date(sell_offer.created_at);
        data.history.push_back(HistoryJSON{
            id,
            owned_by: sell_offer.stonker.clone(),
            issued_by: sell_offer.company.clone(),
            bought_for: sell_offer.threshold,
            created_at: created_at,
            sold_for: sell_offer.threshold,
        });
    }
}