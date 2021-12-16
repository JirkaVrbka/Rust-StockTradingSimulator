use std::fs::File;
use std::io::BufReader;

use csv::ReaderBuilder;
use serde::Deserialize;
use chrono::NaiveDateTime;
use crate::json::NewsJSON;
use crate::json::Effect;

#[derive(Debug, Deserialize)]
struct Glue {
    effect: i8,
    text: String
}

#[derive(Debug, Deserialize)]
struct Titles {
    effect: i8,
    text: String
}

#[derive(Debug, Deserialize)]
struct Headlines {
    date: i32,
    text: String
}

pub struct Generator {
    last_id: i32
}

fn read_csv<T: for<'de> Deserialize<'de>>(fname: &str, del: u8) -> Result<Vec<T>, Box<dyn std::error::Error>> {
    let file = File::open(fname)?;
    let reader = BufReader::new(file);
    let mut csv_reader = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(del)
        .from_reader(reader);
    let mut vec = Vec::new();
    for result in csv_reader.deserialize() {
        let record: T = result?;
        vec.push(record)
    }
    Ok(vec)
}

impl Generator {
    pub fn new() -> Generator {
        let glue = read_csv::<Glue>("./utils/datasets/news/glue.csv", b';');
        println!("{:#?}", glue);
        Generator {
            last_id: -1
        }
    }
    pub fn create(&mut self) -> NewsJSON {
        self.last_id += 1;
        NewsJSON {
            id: self.last_id,
            title: "A".to_string(),
            description: "B".to_string(),
            author: "C".to_string(),
            effect: Effect::NEUTRAL,
            created_at: NaiveDateTime::from_timestamp(0, 0)
        }
    }
}