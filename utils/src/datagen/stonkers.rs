use rand::Rng;
use serde::Deserialize;

use crate::json::StonkerJSON;

use super::read_csv;

#[derive(Deserialize)]
struct Name {
    name: String
}

pub struct Generator {
    last_id: i32,
    names: Vec<Name>,
    random: rand::rngs::ThreadRng,
}

impl Generator {
    pub fn new() -> anyhow::Result<Generator> {
        Ok(Generator {
            names: read_csv::<Name>("names.csv", b';')?,
            last_id: -1,
            random: rand::thread_rng()
        })
    }

    pub fn generate(&mut self) -> Vec<StonkerJSON> {
        let mut stonkers = Vec::new();
        for name in &self.names {
            self.last_id += 1;
            stonkers.push(StonkerJSON {
                id: self.last_id,
                name: name.name.clone(),
                balance: self.random.gen_range(100..100000),
                blocked_balance: 0,
                invested_balance: 0
            })
        }
        stonkers
    }
}

