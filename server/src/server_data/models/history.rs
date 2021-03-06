use crate::schema::history;
use chrono::naive::serde::ts_seconds;
use serde::{Deserialize, Serialize};
use crate::models::stock::Stock;
use crate::models::stonker::Stonker;

#[derive(Queryable, Serialize, Deserialize, Clone, Associations, Identifiable, PartialEq)]
#[table_name = "history"]
#[belongs_to(Stonker)]
#[belongs_to(Stock)]
pub struct History {
    pub id: i32,
    pub stonker_id: i32,
    pub stock_id: i32,
    pub bought_for: i32,
    #[serde(with = "ts_seconds")]
    pub created_at: chrono::NaiveDateTime,
    pub sold_for: i32,
}

#[derive(Insertable, Serialize, Deserialize, Clone)]
#[table_name = "history"]
pub struct NewHistory {
    pub stonker_id: i32,
    pub stock_id: i32,
    pub bought_for: i32,
    pub sold_for: i32,
}
