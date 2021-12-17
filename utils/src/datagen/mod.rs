pub mod news;
pub mod stonkers;
pub mod company;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use anyhow::Error;
use chrono::{Utc, NaiveDateTime};
use csv::ReaderBuilder;
use rand::{thread_rng, Rng, prelude::SliceRandom};
use serde::Deserialize;

#[derive(Debug)]
pub(crate) struct Generator {
    last_id: i32,
    random: rand::rngs::ThreadRng,
}

pub type IndexVec<T> = Vec<(T, bool)>;

fn convert<T>(vec: Vec<T>) -> IndexVec<T> {
    let mut converted = IndexVec::new();
    for item in vec {
        converted.push((item, false))
    }
    converted
}


impl Generator {
    pub fn new() -> Generator {
        Generator {
            last_id: -1,
            random: thread_rng()
        }
    }
    pub fn next(&mut self) -> i32 {
        self.last_id += 1;
        self.last_id
    }
    fn reset<T>(vec: &mut IndexVec<T>) {
        vec.iter_mut().for_each(|(_, used)| *used = true);
    }
    fn free_index<T>(&mut self, vec: &IndexVec<T>) -> Option<usize> {
        let index = self.random.gen_range(0..vec.len());
        for i in 0..vec.len() {
            let (_, used) = &vec[index + i];
            if !used {
                return Some(index + i);
            }
        }
        None
    }
    pub fn choose<'a, T: 'a>(&mut self, collection: &'a mut IndexVec<T>) -> &'a T {
        if collection.is_empty() {
            panic!("Empty collection");
        }
        match self.free_index(collection) {
            None => {
                Generator::reset(collection);
                &collection.choose(&mut self.random).unwrap().0
            }
            Some(val) => &collection[val].0
        }
    }
    pub fn choose_from<'a, T, K>(&mut self, map: &'a mut HashMap<K, IndexVec<T>>, key: &K) -> &'a T where T: 'a, K: std::cmp::Eq + std::hash::Hash {
        self.choose(map.get_mut(&key).expect("Map doesn't contain the key"))
    }
    pub fn date_from_days(&mut self, days: usize) -> NaiveDateTime {
        let seconds = (days*24*60*60) as i64;
        let now = Utc::now().naive_utc().timestamp();
        let at = self.random.gen_range((now-seconds)..now);
        NaiveDateTime::from_timestamp(at, 0)
    }
}

pub fn read_csv<T: for<'de> Deserialize<'de>>(fname: &str, del: u8) -> Result<IndexVec<T>, Error> {
    let file = File::open(format!("./utils/datasets/{}", fname))?;
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
    Ok(convert(vec))
}