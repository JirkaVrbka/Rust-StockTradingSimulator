use crate::{json::{HistoryJSON, CommandTypesJSON}, datagen::ToTSQLValue};

use super::{Generator, Data, JsonGenerator, ToTSQL};

impl ToTSQL for HistoryJSON {
    fn to_header() -> &'static str {
        "History"
    }
    fn to_columns() -> Vec<&'static str> {
        vec!["id", "stonker_id", "stock_id", "bought_for", "created_at", "sold_for"]
    }
    fn to_data(&self) -> Vec<super::TSQLValue> {
        vec![self.id.to_id(), self.owned_by.id.to_id(),
            self.issued_by.id.to_id(), self.bought_for.to(),
            self.created_at.to(), self.sold_for.to()]
    }
}

pub struct HistoryGenerator;

impl JsonGenerator for  HistoryGenerator {
    fn new() -> anyhow::Result<HistoryGenerator> {
        Ok(HistoryGenerator)
    }
    fn create(&mut self, generator: &mut Generator, data: &mut Data) {
        // choose random SELL command and sell it to random stonker
        let id = data.next();
        let command = generator.choose_mut(&mut data.commands);
        match command.kind {
            CommandTypesJSON::Sell => (),
            _ => panic!("Can create history only for sell commands")
        }
        let buyer = generator.choose_mut(&mut data.stonkers);
        let mut lowest_price = i32::MAX;
        let mut stock = None;
        for some_stock in data.stocks.iter_mut() {
            if some_stock.owner.id == command.stonker.id && some_stock.bought_for < lowest_price {
                lowest_price = some_stock.bought_for;
                stock = Some(some_stock);
            }
        }
        let stock = stock.expect(format!("Could not find stock for command {:?}", command).as_str());
        let bought_for = stock.bought_for;
        let sold_for = command.threshold;
        buyer.balance -= command.threshold;
        stock.bought_for = command.threshold;
        stock.owner = buyer.clone();
        command.threshold = generator.random_price(sold_for, buyer.balance / 10);
        command.stonker = buyer.clone();
        command.created_at = generator.random_date(command.created_at);
        data.history.push_back(HistoryJSON {
            id,
            owned_by: command.stonker.clone(),
            issued_by: command.company.clone(),
            bought_for,
            created_at: command.created_at,
            sold_for,
        })
    }
}