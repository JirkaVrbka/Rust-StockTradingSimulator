use std::collections::HashMap;
use serde::Deserialize;
use crate::json::NewsJSON;
use anyhow::Error;
use crate::json::EffectJSON;
use strum::IntoEnumIterator;
use super::Data;
use super::IndexVec;
use super::Generator;
use super::ToTSQL;

fn effect_to_string(effect: EffectJSON) ->&'static str {
    match effect {
        EffectJSON::Fall => "FALL",
        EffectJSON::Neutral => "NEUTRAL",
        EffectJSON::Rise => "RISE",
    }
}

impl ToTSQL for NewsJSON {
    fn to_header() -> &'static str {
        "News"
    }
    fn to_columns() -> Vec<&'static str> {
        vec!["id", "title", "description", "author", "created_at", "kind", "company_id"]
    }
    fn to_data(&self) -> Vec<String> {
        vec![self.id.to_string(), self.title.to_string(), self.description.to_string(),
            self.author.to_string(), self.created_at.to_string(),
            effect_to_string(self.effect.clone()).to_string(),
            self.company.id.to_string()]
    }
}


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
}

fn into_map(vec: IndexVec<Info>) -> HashMap<EffectJSON, IndexVec<String>> {
    let mut map = HashMap::new();
    for Info { effect, text } in vec {
        map.entry(effect).or_insert_with(IndexVec::new).push_back(text);
    }
    map
}

impl NewsGenerator {
    pub fn new() -> Result<NewsGenerator, Error>  {
        Ok(NewsGenerator {
            generator: Generator::new(),
            glues: into_map(IndexVec::read_csv("news/glue.csv", b';')?),
            titles: into_map(IndexVec::read_csv("news/titles.csv", b';')?),
            newspapers: IndexVec::read_csv("news/newspapers.csv", b',')?,
            headlines: IndexVec::read_csv("news/headlines.csv", b',')?,
            effects: IndexVec::convert(EffectJSON::iter().collect::<Vec<EffectJSON>>()),
        })
    }

    pub fn create(&mut self, data: &mut Data) {
        let company = self.generator.choose(&mut data.companies).clone();
        let effect = self.generator.choose(&mut self.effects).clone();
        let glue = self.generator.choose_from(&mut self.glues, &effect).replace("{}", company.performer.name.as_str());
        let headline = self.generator.choose(&mut self.headlines).text.clone();
        let first_char = headline.chars().next().expect("Headline is empty");
        let headline = format!("{}{}", first_char.to_uppercase(), headline.chars().skip(1).collect::<String>());
        let recently = self.generator.date_from_days(3);
        data.news.push_back(NewsJSON {
            id: self.generator.next(),
            title: self.generator.choose_from(&mut self.titles, &effect).replace("{}", company.name.as_str()).clone(),
            description: format!("{}{}", headline, glue),
            author: self.generator.choose(&mut self.newspapers).name.clone(),
            effect,
            created_at: recently,
            company
        });
    }
}
