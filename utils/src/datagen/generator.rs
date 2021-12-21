use std::collections::HashMap;
use chrono::{Utc, NaiveDateTime};
use rand::{thread_rng, Rng, };
use super::index_vec::IndexVec;

#[derive(Debug)]
pub struct Generator {
    last_id: i32,
    pub random: rand::rngs::ThreadRng,
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
        let index = self.random.gen_range(0..collection.len());
        collection.choose(index)
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
