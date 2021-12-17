use std::collections::HashMap;
use rand::Rng;
use serde::Deserialize;
use chrono::{NaiveDateTime, Utc};
use crate::datagen::read_csv;
use crate::json::NewsJSON;
use anyhow::Error;
use rand::seq::SliceRandom;
use crate::json::Effect;
use strum::IntoEnumIterator;

#[derive(Debug, Deserialize)]
struct Info {
    effect: Effect,
    text: String
}

#[derive(Debug, Deserialize)]
struct Headline {
    date: i32,
    text: String
}

#[derive(Debug, Deserialize)]
struct Newspaper {
    name: String
}

#[derive(Debug)]
pub struct Generator {
    last_id: i32,
    glues: HashMap<Effect, Vec<String>>,
    titles: HashMap<Effect, Vec<String>>,
    newspapers: Vec<Newspaper>,
    headlines: Vec<Headline>,
    rng: rand::rngs::ThreadRng,
}

fn into_map(vec: Vec<Info>) -> HashMap<Effect, Vec<String>> {
    let mut map = HashMap::new();
    for Info { effect, text } in vec {
        map.entry(effect).or_insert_with(Vec::new).push(text)
    }
    map
}

impl Generator {
    pub fn new() -> Result<Generator, Error>  {
        Ok(Generator {
            last_id: -1,
            glues: into_map(read_csv::<Info>("news/glue.csv", b';')?),
            titles: into_map(read_csv::<Info>("news/titles.csv", b';')?),
            newspapers: read_csv::<Newspaper>("news/newspapers.csv", b',')?,
            headlines: read_csv::<Headline>("news/headlines.csv", b',')?,
            rng: rand::thread_rng()
        })
    }

    pub fn create(&mut self) -> NewsJSON {
        self.last_id += 1;
        let effects = Effect::iter().collect::<Vec<Effect>>();
        let effect = effects.choose(&mut self.rng).expect("Effects are empty").clone();
        let title = self.titles.get(&effect).expect(format!("No title for {:?}", effect).as_str()).choose(&mut self.rng).expect("Titles are empty").clone();
        let glue = self.glues.get(&effect).expect(format!("No glue for {:?}", effect).as_str()).choose(&mut self.rng).expect("Glues are empty").clone();
        let author = self.newspapers.choose(&mut self.rng).expect("Newspapers are empty").name.clone();
        let headline = self.headlines.choose(&mut self.rng).expect("Headlines are empty").text.clone();
        let first_char = headline.chars().next().expect("Headline is empty");
        let headline = format!("{}{}", first_char.to_uppercase(), headline.chars().skip(1).collect::<String>());
        let three_days = 3*24*60*60;
        let now = Utc::now().naive_utc().timestamp();
        let at = self.rng.gen_range((now-three_days)..now);
        NewsJSON {
            id: self.last_id,
            title,
            description: format!("{}{}", headline, glue),
            author,
            effect,
            created_at: NaiveDateTime::from_timestamp(at, 0)
        }
    }
}