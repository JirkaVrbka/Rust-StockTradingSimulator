use rand::Rng;
use serde::Deserialize;
use crate::json::StonkerJSON;
use super::{read_csv, Generator};

#[derive(Deserialize)]
struct Name {
    first: String,
    last: String,
}

pub struct StonkerGenerator {
    generator: Generator,
    first_names: Vec<String>,
    last_names: Vec<String>,
}

impl StonkerGenerator {
    pub fn new() -> anyhow::Result<StonkerGenerator> {
        let names = read_csv::<Name>("names.csv", b' ')?;
        Ok(StonkerGenerator {
            generator: Generator::new(),
            first_names: names.iter().map(|name| name.first.clone()).collect(),
            last_names: names.iter().map(|name| name.last.clone()).collect(),
        })
    }

    pub fn create(&mut self) -> StonkerJSON {
        StonkerJSON {
            id: self.generator.next(),
            name: format!("{} {}", self.generator.choose(&self.first_names), self.generator.choose(&self.last_names)),
            balance: self.generator.random.gen_range(100..100000),
            blocked_balance: 0,
            invested_balance: 0
        }
    }
}

