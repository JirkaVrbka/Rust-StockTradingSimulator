use crate::{schema::news, server_data::repos::Repo};
use diesel_derive_enum::DbEnum;
use chrono::naive::serde::ts_seconds;
use serde::{Deserialize, Serialize};
use utils::json::{EffectJSON, NewsJSON};
use crate::server_data::models::company::Company;
use crate::schema::company::dsl::company;

use super::{ToJson, Connection, FromJson};

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

impl FromJson<EffectJSON> for Effect {
    fn from_json(json: &EffectJSON) -> Self {
        match json {
            EffectJSON::Fall => Effect::Fall,
            EffectJSON::Neutral => Effect::Neutral,
            EffectJSON::Rise => Effect::Rise,
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

impl ToJson<NewsJSON> for News {
    fn to_json(&self, connection: &Connection) -> anyhow::Result<NewsJSON> {
        let affected = Repo::find::<Company, _>(
            &connection,
            company,
            self.company_id,
            format!("company of news {}", self.id).as_str()
        )?;
        Ok(NewsJSON {
            id: self.id,
            title: self.title.clone(),
            description: self.description.clone(),
            author: self.author.clone(),
            created_at: self.created_at,
            effect: self.kind.to_json(connection)?,
            company: affected.to_json(connection)?
        })
    }
}
