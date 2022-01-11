use crate::json::{HistoryJSON, CommandJSON, StonkerJSON};

use super::{Generator, IndexVec, Data};

pub struct HistoryGenerator;

impl HistoryGenerator {
    pub fn new() -> HistoryGenerator {
        HistoryGenerator { }
    }
    pub fn create_history(&mut self, generator: &mut Generator, data: &mut Data) {
        let id = data.next();
        let sell_offer = generator.choose(&mut data.commands);
        let buyer = generator.choose(&mut data.stonkers).clone();
        let created_at = generator.random_date(sell_offer.created_at);
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