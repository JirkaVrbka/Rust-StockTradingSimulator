use rand::Rng;
use serde::Deserialize;
use crate::json::StonkerJSON;
use super::{Generator, IndexVec};

#[derive(Deserialize)]
struct Name {
    first: String,
    last: String,
}

pub struct StonkerGenerator {
    generator: Generator,
    first_names: IndexVec<String>,
    last_names: IndexVec<String>,
    spawned: IndexVec<StonkerJSON>
}

impl StonkerGenerator {
    pub fn new() -> anyhow::Result<StonkerGenerator> {
        let names: IndexVec<Name> = IndexVec::read_csv("names.csv", b' ')?;
        Ok(StonkerGenerator {
            generator: Generator::new(),
            first_names: IndexVec::from(&names, |name| name.first.clone()),
            last_names: IndexVec::from(&names, |name| name.last.clone()),
            spawned: IndexVec::new()
        })
    }

    pub fn create(&mut self) -> &StonkerJSON {
        self.spawned.push_back(StonkerJSON {
            id: self.generator.next(),
            name: format!("{} {}", self.generator.choose(&mut self.first_names), self.generator.choose(&mut self.last_names)),
            balance: self.generator.random.gen_range(100..100000),
            blocked_balance: 0,
            invested_balance: 0
        })
    }
}

