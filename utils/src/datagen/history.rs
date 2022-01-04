use crate::json::{HistoryJSON, CommandJSON, StonkerJSON};

use super::{Generator, IndexVec};

pub struct HistoryGenerator {
    generator: Generator,
    spawned: IndexVec<HistoryJSON>
}

impl HistoryGenerator {
    pub fn new() -> HistoryGenerator {
        HistoryGenerator {
            generator: Generator::new(),
            spawned: IndexVec::new(),
        }
    }
    pub fn create_history(&mut self, stonkers: &mut IndexVec<StonkerJSON>, sell_offer: &CommandJSON) -> &HistoryJSON {
        let id = self.generator.next();
        let created_at = self.generator.random_date(sell_offer.created_at);
        let buyer = self.generator.choose(stonkers).clone();
        let history = HistoryJSON{
            id: self.generator.next(),
            owned_by: sell_offer.stonker.clone(),
            issued_by: sell_offer.company.clone(),
            bought_for: sell_offer.threshold,
            created_at: created_at,
            sold_for: sell_offer.threshold,
        };
        self.spawned.push_back(history)
    }
}