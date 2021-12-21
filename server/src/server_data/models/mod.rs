use diesel::{r2d2::{PooledConnection, ConnectionManager}, PgConnection};

pub mod command;
pub mod company;
pub mod history;
pub mod news;
pub mod stock;
pub mod stonker;

pub type Connection = PooledConnection<ConnectionManager<PgConnection>>;

pub trait ToJson<T> {
    fn to_json(&self, connection: &Connection) -> anyhow::Result<T>;
}

pub trait FromJson<T> {
    fn from_json(json: &T) -> Self;
}

impl<T, J: ToJson<T>> ToJson<Vec<T>> for Vec<J> {
    fn to_json(&self, connection: &Connection) -> anyhow::Result<Vec<T>> {
        self
            .iter()
            .map(|entity| entity.to_json(connection))
            .collect()
    }
}

impl<T, J: FromJson<T>> FromJson<Vec<T>> for Vec<J> {
    fn from_json(json: &Vec<T>) -> Self {
        json.iter().map(|j| J::from_json(j)).collect()
    }
}