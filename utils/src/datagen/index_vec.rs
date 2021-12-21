use std::fs::File;
use std::io::BufReader;
use anyhow::Error;
use csv::ReaderBuilder;
use serde::Deserialize;
use std::iter::Iterator;

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
    fn next(&self, index: usize) -> Option<usize> {
        for i in 0..self.0.len() {
            let (_, used) = &self.0[index + i];
            if !used {
                return Some(index + i);
            }
        }
        None
    }
    pub fn choose(&mut self, index: usize) -> &T {
        if self.0.is_empty() {
            panic!("Empty collection");
        }
        match self.next(index) {
            None => {
                self.reset();
                &self.0[index].0
            }
            Some(val) => &self.0[val].0
        }
    }
    pub fn len(&self) -> usize {
        self.0.len()
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