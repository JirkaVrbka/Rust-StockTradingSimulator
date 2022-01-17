use rand::Rng;

use crate::{json::{StockJSON, CommandJSON, CommandTypesJSON}, datagen::ToTSQLValue};

use super::{Generator, Data, JsonGenerator, ToTSQL};

impl ToTSQL for StockJSON {
    fn to_header() -> &'static str {
        "Stock"
    }
    fn to_columns() -> Vec<&'static str> {
        vec!["id", "stonker_id", "company_id", "share", "bought_for"]
    }
    fn to_data(&self) -> Vec<super::TSQLValue> {
        vec![self.id.to_id(), self.owner.id.to_id(), self.issued_by.id.to_id(), self.share.to(), self.bought_for.to()]
    }
}

pub struct StockGenerator;

impl JsonGenerator for StockGenerator {
    fn new() -> anyhow::Result<StockGenerator> {
        Ok(StockGenerator)
    }
    fn create(&mut self, generator: &mut Generator, data: &mut Data) {
        let count = data.next();
        let company = generator.choose(&mut data.companies);
        let stocks = generator.random.gen_range(1..100); // 1 - 99 stocks
        let share = generator.random.gen_range(1..(1_000_000/stocks));
        let threshold = company.performer.invested_balance;
        for nth in 0..stocks {
            data.stocks.push_back(
            StockJSON {
                id: count + 2 * nth,
                owner: company.performer.clone(),
                issued_by: company.clone(),
                share,
                bought_for: 0,
            });
            data.commands.push_back({
                CommandJSON {
                    id: count + 2 * nth + 1,
                    stonker: company.performer.clone(),
                    company: company.clone(),
                    threshold,
                    share,
                    kind: CommandTypesJSON::Sell,
                    created_at: Generator::get_beginning()
                }
            })
        }
    }
}