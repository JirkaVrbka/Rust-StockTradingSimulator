use std::collections::HashMap;
use rand::Rng;
use serde::Deserialize;
use chrono::{NaiveDateTime, Utc};
use crate::datagen::read_csv;
use crate::json::{NewsJSON, CompanyJSON, StonkerJSON};
use anyhow::Error;
use rand::seq::SliceRandom;
use crate::json::EffectJSON;
use strum::IntoEnumIterator;

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
    glues: HashMap<EffectJSON, Vec<String>>,
    titles: HashMap<EffectJSON, Vec<String>>,
    newspapers: Vec<Newspaper>,
    headlines: Vec<Headline>,
}

fn into_map(vec: Vec<Info>) -> HashMap<EffectJSON, Vec<String>> {
    let mut map = HashMap::new();
    for Info { effect, text } in vec {
        map.entry(effect).or_insert_with(Vec::new).push(text)
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
        })
    }

    pub fn create(&mut self) -> NewsJSON {
        let effects = EffectJSON::iter().collect::<Vec<EffectJSON>>();
        let effect = self.generator.choose(&effects).clone();
        let glue = self.generator.choose_from(&self.glues, &effect).clone();
        let headline = self.generator.choose(&self.headlines).text.clone();
        let first_char = headline.chars().next().expect("Headline is empty");
        let headline = format!("{}{}", first_char.to_uppercase(), headline.chars().skip(1).collect::<String>());
        let recently = self.generator.date_from_days(3);
        NewsJSON {
            id: self.generator.next(),
            title: self.generator.choose_from(&self.titles, &effect).clone(),
            description: format!("{}{}", headline, glue),
            author: self.generator.choose(&self.newspapers).name.clone(),
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
        }
    }
}