use crate::schema::news;
use chrono::naive::serde::ts_seconds;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Clone, Associations, Identifiable, PartialEq)]
#[table_name = "news"]
pub struct News {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub author: String,
    #[serde(with = "ts_seconds")]
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize, Clone)]
#[table_name = "news"]
pub struct NewNews {
    pub title: String,
    pub description: String,
    pub author: String,
}
