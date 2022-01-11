use std::collections::HashMap;
use chrono::{Utc, NaiveDateTime};
use rand::{thread_rng, Rng};
use super::index_vec::IndexVec;
use rand_distr::{Normal, Exp};

#[derive(Debug)]
pub struct Generator {
    pub random: rand::rngs::ThreadRng,
}

impl Generator {
    pub fn new() -> Generator {
        Generator {
            random: thread_rng()
        }
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
    // we want to have > 0.95 percentage certainty that the
    // transaction happened in interval [last_transaction, last_transaction/2)
    // Wolfram: CDF[ExponentialDistribution[6], 0.5] = 0.950213
    pub fn random_date(&mut self, last: NaiveDateTime) -> NaiveDateTime {
        let now = chrono::offset::Local::now();
        let difference = (now.timestamp() - last.timestamp()) as f64;
        let exponential = Exp::new(6.0).unwrap();
        let created_at = self.random.sample(exponential) * difference;
        NaiveDateTime::from_timestamp(created_at.round() as i64, 0)
    }
    pub fn random_price(&mut self, last: i32, wealth: i32) -> i32 {
        // we want to have > 0.95 percentage certainty
        // that stonker had enough money to buy the stock
        // Wolfram: CDF[NormalDistribution[0, 1], 2] - CDF[NormalDistribution[0, 1], -2] = 0.9545
        let normal = Normal::new(last as f64, (wealth as f64) / 2.0).unwrap();
        self.random.sample(normal).round() as i32
    }

    pub fn random_passwd(&mut self, len: i32) -> String {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";

        let password: String = (0..len)
            .map(|_| {
                let idx = self.random.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();
        password
    }
}
