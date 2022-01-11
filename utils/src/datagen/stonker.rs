use rand::Rng;
use serde::Deserialize;
use crate::{json::StonkerJSON, datagen::{ToTSQLValue, TSQLValue}};
use super::{Generator, IndexVec, Data, ToTSQL, JsonGenerator};

impl ToTSQL for StonkerJSON {
    fn to_header() -> &'static str {
        "Stonker"
    }
    fn to_columns() -> Vec<&'static str> {
        vec!["id", "name", "password", "balance", "blocked_balance", "invested_balance"]
    }
    fn to_data(&self) -> Vec<TSQLValue> {
        vec![self.id.to_id(), self.name.to(), self.password.to(),
            self.balance.to(), self.blocked_balance.to(),
            self.invested_balance.to()]
    }
}

#[derive(Deserialize)]
struct Name {
    first: String,
    last: String,
}

pub struct StonkerGenerator {
    first_names: IndexVec<String>,
    last_names: IndexVec<String>,
}

impl JsonGenerator for StonkerGenerator {
    fn new() -> anyhow::Result<StonkerGenerator> {
        let names: IndexVec<Name> = IndexVec::read_csv("names.csv", b' ')?;
        Ok(StonkerGenerator {
            first_names: IndexVec::from(&names, |name| name.first.clone()),
            last_names: IndexVec::from(&names, |name| name.last.clone()),
        })
    }

    fn create(&mut self, generator: &mut Generator, data: &mut Data) {
        data.stonkers.push_back(StonkerJSON {
            id: data.next(),
            name: format!("{} {}", generator.choose(&mut self.first_names), generator.choose(&mut self.last_names)),
            balance: generator.random.gen_range(100..100000),
            blocked_balance: 0,
            invested_balance: 0,
            password: generator.random_passwd(5),
        })
    }
}
