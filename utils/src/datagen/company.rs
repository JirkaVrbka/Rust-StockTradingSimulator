use rand::Rng;
use serde::Deserialize;

use crate::json::{CompanyJSON, StonkerJSON};

use super::{Generator, IndexVec, Data, ToTSQL, TSQLValue, ToTSQLValue, JsonGenerator};

impl ToTSQL for CompanyJSON {
    fn to_header() -> &'static str {
        "Company"
    }
    fn to_columns() -> Vec<&'static str> {
        vec!["id", "name", "performer_id"]
    }
    fn to_data(&self) -> Vec<TSQLValue> {
        vec![self.id.to_id(), self.name.to(), self.performer.id.to_id()]
    }
}

#[derive(Debug, Deserialize)]
struct Stock {
    symbol: String,
    name: String,
    price: f64
}

pub struct CompanyGenerator {
    stocks: IndexVec<Stock>,
}

impl JsonGenerator for CompanyGenerator {
    fn new() -> anyhow::Result<CompanyGenerator> {
        Ok(CompanyGenerator{
            stocks: IndexVec::read_csv("stocks.csv", b',')?,
        })
    }
    fn create(&mut self, generator: &mut Generator, data: &mut Data) {
        let stock = generator.choose(&mut self.stocks);
        let performer = StonkerJSON {
            id: data.next(),
            name: stock.name.clone(),
            balance: generator.random.gen_range(10_000..100_000),
            blocked_balance: 0,
            invested_balance: 0,
            password: generator.random_passwd(5),
        };
        data.stonkers.push_back(performer.clone());
        data.companies.push_back(CompanyJSON {
            id: data.next(),
            name: stock.symbol.clone(),
            performer
        });
    }
}
