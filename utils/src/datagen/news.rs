use std::collections::HashMap;
use serde::Deserialize;
use crate::datagen::read_csv;
use crate::json::{NewsJSON, CompanyJSON, StonkerJSON};
use anyhow::Error;
use crate::json::EffectJSON;
use strum::IntoEnumIterator;
use super::{IndexVec, convert, push_back};
use super::Generator;

#[derive(Debug, Deserialize)]
struct Info {
    effect: EffectJSON,
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
pub struct NewsGenerator {
    generator: Generator,
    glues: HashMap<EffectJSON, IndexVec<String>>,
    titles: HashMap<EffectJSON, IndexVec<String>>,
    newspapers: IndexVec<Newspaper>,
    headlines: IndexVec<Headline>,
    effects: IndexVec<EffectJSON>,
    spawned: IndexVec<NewsJSON>,
}

fn into_map(vec: IndexVec<Info>) -> HashMap<EffectJSON, IndexVec<String>> {
    let mut map = HashMap::new();
    for (Info { effect, text }, used) in vec {
        map.entry(effect).or_insert_with(Vec::new).push((text, used))
    }
    map
}

impl NewsGenerator {
    pub fn new() -> Result<NewsGenerator, Error>  {
        Ok(NewsGenerator {
            generator: Generator::new(),
            glues: into_map(read_csv::<Info>("news/glue.csv", b';')?),
            titles: into_map(read_csv::<Info>("news/titles.csv", b';')?),
            newspapers: read_csv::<Newspaper>("news/newspapers.csv", b',')?,
            headlines: read_csv::<Headline>("news/headlines.csv", b',')?,
            effects: convert(EffectJSON::iter().collect::<Vec<EffectJSON>>()),
            spawned: IndexVec::new(),
        })
    }

    pub fn create(&mut self) -> &NewsJSON {
        let effect = self.generator.choose(&mut self.effects).clone();
        let glue = self.generator.choose_from(&mut self.glues, &effect).clone();
        let headline = self.generator.choose(&mut self.headlines).text.clone();
        let first_char = headline.chars().next().expect("Headline is empty");
        let headline = format!("{}{}", first_char.to_uppercase(), headline.chars().skip(1).collect::<String>());
        let recently = self.generator.date_from_days(3);
        push_back(&mut self.spawned, NewsJSON {
            id: self.generator.next(),
            title: self.generator.choose_from(&mut self.titles, &effect).clone(),
            description: format!("{}{}", headline, glue),
            author: self.generator.choose(&mut self.newspapers).name.clone(),
            effect,
            created_at: recently,
            company: CompanyJSON {
                id: 1,
                name: "Netflix".to_string(),
                performer: StonkerJSON {
                    id: 0,
                    name: "Netflixer".to_string(),
                    balance: 0,
                    blocked_balance: 0,
                    invested_balance: 0,
                }
            }
        })
    }
}