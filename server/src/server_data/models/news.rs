use crate::schema::news;
use diesel_derive_enum::DbEnum;
use chrono::naive::serde::ts_seconds;
use serde::{Deserialize, Serialize};
use utils::json::EffectJSON;

use super::ToJson;

#[derive(Serialize, Deserialize, Clone, DbEnum, Debug, PartialEq)]
#[DieselType = "Effectdb"]
#[DbValueStyle = "SCREAMING_SNAKE_CASE"]
pub enum Effect {
    Fall,
    Neutral,
    Rise,
}

impl ToJson<EffectJSON> for Effect {
    fn to_json(&self, _: &super::Connection) -> anyhow::Result<EffectJSON> {
        match self {
            Effect::Fall => Ok(EffectJSON::Fall),
            Effect::Neutral => Ok(EffectJSON::Neutral),
            Effect::Rise => Ok(EffectJSON::Rise),
        }
    }
}

#[derive(Debug, Queryable, Serialize, Deserialize, Clone, Associations, Identifiable, PartialEq)]
#[table_name = "news"]
pub struct News {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub author: String,
    #[serde(with = "ts_seconds")]
    pub created_at: chrono::NaiveDateTime,
    pub kind: Effect,
    pub company_id: i32,
}
