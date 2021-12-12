use crate::schema::history;
use chrono::naive::serde::ts_seconds;
use serde::{Deserialize, Serialize};

use super::{company::CompanyJSON, stonker::Stonker};

#[derive(Queryable, Serialize, Deserialize, Clone, Associations, Identifiable, PartialEq)]
#[table_name = "history"]
pub struct History {
    pub id: i32,
    pub stonker_id: i32,
    pub stock_id: i32,
    pub bought_for: Option<i32>,
    #[serde(with = "ts_seconds")]
    pub created_at: chrono::NaiveDateTime,
    pub sold_for: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct HistoryJSON {
    pub id: i32,
    pub owned_by: Stonker,
    pub issued_by: CompanyJSON,
    pub bought_for: Option<i32>,
    #[serde(with = "ts_seconds")]
    pub created_at: chrono::NaiveDateTime,
    pub sold_for: Option<i32>,
}

#[derive(Insertable, Serialize, Deserialize, Clone)]
#[table_name = "history"]
pub struct NewHistory {
    pub stonker_id: i32,
    pub stock_id: i32,
    pub bought_for: Option<i32>,
    pub sold_for: Option<i32>,
}