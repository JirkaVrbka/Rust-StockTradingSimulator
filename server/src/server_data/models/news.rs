use crate::schema::news;
use chrono::naive::serde::ts_seconds;
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
use utils::json::EffectJSON;

#[derive(Serialize, Deserialize, Clone, DbEnum, Debug, PartialEq)]
#[DieselType = "Effectdb"]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Effect {
    FALL,
    NEUTRAL,
    RISE,
}

impl Effect {
    pub fn from_json(json: EffectJSON) -> Effect {
        match json {
            EffectJSON::FALL => Effect::FALL,
            EffectJSON::NEUTRAL => Effect::NEUTRAL,
            EffectJSON::RISE => Effect::RISE,
        }
    }
    pub fn to_json(&self) -> EffectJSON {
        match self {
            Effect::FALL => EffectJSON::FALL,
            Effect::NEUTRAL => EffectJSON::NEUTRAL,
            Effect::RISE => EffectJSON::RISE,
        }
    }
}


#[derive(Queryable, Serialize, Deserialize, Clone, Associations, Identifiable, PartialEq)]
#[table_name = "news"]
pub struct News {
    pub id: i32,
    pub company_id: i32,
    pub title: String,
    pub description: String,
    pub author: String,
    #[serde(with = "ts_seconds")]
    pub created_at: chrono::NaiveDateTime,
    pub effect: Effect,
}
