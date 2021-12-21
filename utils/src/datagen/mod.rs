pub mod news;
pub mod stonkers;
pub mod company;
pub mod history;

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use anyhow::Error;
use chrono::{Utc, NaiveDateTime};
use csv::ReaderBuilder;
use rand::{thread_rng, Rng, prelude::SliceRandom};
use serde::Deserialize;
use std::iter::Iterator;

#[derive(Debug)]
pub struct Generator {
    last_id: i32,
    random: rand::rngs::ThreadRng,
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
    pub fn choose<'a, T: 'a>(&mut self, collection: &'a mut IndexVec<T>) -> &'a T {
        collection.choose(self)
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

#[derive(Debug)]
pub struct IndexVec<T>(Vec<(T, bool)>);

impl<T> IndexVec<T> {
    pub fn new() -> IndexVec<T> {
        IndexVec(Vec::new())
    }
    pub fn convert(vec: Vec<T>) -> IndexVec<T> {
        let mut converted = IndexVec::new();
        for item in vec {
            converted.push_back(item);
        }
        converted
    }
    pub fn from<J>(vec: &IndexVec<T>, convert: fn(&T)->J) -> IndexVec<J> {
        IndexVec(vec.0.iter().map(|(value, used)| (convert(value), *used)).collect())
    }
    pub fn push_back(&mut self, item: T) -> &T {
        self.0.push((item, false));
        &self.0.last().unwrap().0
    }
    fn reset(&mut self) {
        self.0.iter_mut().for_each(|(_, used)| *used = true);
    }
    fn free_index(&self, gen: &mut Generator) -> Option<usize> {
        let index = gen.random.gen_range(0..self.0.len());
        for i in 0..self.0.len() {
            let (_, used) = &self.0[index + i];
            if !used {
                return Some(index + i);
            }
        }
        None
    }
    pub fn choose(&mut self, generator: &mut Generator) -> &T {
        if self.0.is_empty() {
            panic!("Empty collection");
        }
        match self.free_index(generator) {
            None => {
                self.reset();
                &self.0.choose(&mut generator.random).unwrap().0
            }
            Some(val) => &self.0[val].0
        }
    }
}

impl<T: for<'de> Deserialize<'de>> IndexVec<T> {
    pub fn read_csv(fname: &str, del: u8) -> Result<IndexVec<T>, Error> {
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
        Ok(IndexVec::convert(vec))
    }
}

impl<T> IntoIterator for IndexVec<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter().map(|(value, _)| value).collect::<Vec<T>>().into_iter()
    }
}