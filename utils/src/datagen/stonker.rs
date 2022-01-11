use rand::Rng;
use serde::Deserialize;
use crate::json::StonkerJSON;
use super::{Generator, IndexVec, Data, ToTSQL};

impl ToTSQL for StonkerJSON {
    fn to_header() -> &'static str {
        "Stonker"
    }
    fn to_columns() -> Vec<&'static str> {
        vec!["id", "name", "balance", "blocked_balance", "invested_balance"]
    }
    fn to_data(&self) -> Vec<String> {
        vec![self.id.to_string(), self.name.to_string(),
            self.balance.to_string(), self.blocked_balance.to_string(),
            self.invested_balance.to_string()]
    }
}

#[derive(Deserialize)]
struct Name {
    first: String,
    last: String,
}

pub struct StonkerGenerator {
    generator: Generator,
    first_names: IndexVec<String>,
    last_names: IndexVec<String>,
}

impl StonkerGenerator {
    pub fn new() -> anyhow::Result<StonkerGenerator> {
        let names: IndexVec<Name> = IndexVec::read_csv("names.csv", b' ')?;
        Ok(StonkerGenerator {
            generator: Generator::new(),
            first_names: IndexVec::from(&names, |name| name.first.clone()),
            last_names: IndexVec::from(&names, |name| name.last.clone()),
        })
    }

    pub fn create(&mut self, data: &mut Data) {
        data.stonkers.push_back(StonkerJSON {
            id: data.next(),
            name: format!("{} {}", self.generator.choose(&mut self.first_names), self.generator.choose(&mut self.last_names)),
            balance: self.generator.random.gen_range(100..100000),
            blocked_balance: 0,
            invested_balance: 0,
            password: self.generator.random_passwd(5),
        })
    }
}
