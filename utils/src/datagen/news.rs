use std::collections::HashMap;
use serde::Deserialize;
use crate::datagen::{ToTSQLValue, TSQLValue};
use crate::json::NewsJSON;
use crate::json::EffectJSON;
use strum::IntoEnumIterator;
use super::{Data, JsonGenerator};
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
    fn to_data(&self) -> Vec<TSQLValue> {
        vec![self.id.to_id(), self.title.to(), self.description.to(),
            self.author.to(), self.created_at.to(),
            effect_to_string(self.effect.clone()).to_string().to(),
            self.company.id.to_id()]
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

impl JsonGenerator for NewsGenerator {
    fn new() -> anyhow::Result<NewsGenerator>  {
        Ok(NewsGenerator {
            glues: into_map(IndexVec::read_csv("news/glue.csv", b';')?),
            titles: into_map(IndexVec::read_csv("news/titles.csv", b';')?),
            newspapers: IndexVec::read_csv("news/newspapers.csv", b',')?,
            headlines: IndexVec::read_csv("news/headlines.csv", b',')?,
            effects: IndexVec::convert(EffectJSON::iter().collect::<Vec<EffectJSON>>()),
        })
    }
    fn create(&mut self, generator: &mut Generator, data: &mut Data) {
        let company = generator.choose(&mut data.companies).clone();
        let effect = generator.choose(&mut self.effects).clone();
        let glue = generator.choose_from(&mut self.glues, &effect).replace("{}", company.performer.name.as_str());
        let headline = generator.choose(&mut self.headlines).text.clone();
        let first_char = headline.chars().next().expect("Headline is empty");
        let headline = format!("{}{}", first_char.to_uppercase(), headline.chars().skip(1).collect::<String>());
        let recently = generator.date_from_days(3);
        data.news.push_back(NewsJSON {
            id: data.next(),
            title: generator.choose_from(&mut self.titles, &effect).replace("{}", company.name.as_str()).clone(),
            description: format!("{}{}", headline, glue),
            author: generator.choose(&mut self.newspapers).name.clone(),
            effect,
            created_at: recently,
            company
        });
    }
}
