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