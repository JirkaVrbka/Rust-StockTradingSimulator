use crate::schema::command;
use chrono::naive::serde::ts_seconds;
use chrono::Utc;
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
use utils::json::CommandTypesJSON;

use super::{ToJson, FromJson};

#[derive(Serialize, Deserialize, Clone, DbEnum, Debug, PartialEq)]
#[DieselType = "Commandtypesdb"]
#[DbValueStyle = "SCREAMING_SNAKE_CASE"]
pub enum CommandTypes {
    Sell,
    SellIfHigh,
    SellIfLow,
    BuyIfLow,
}

impl ToJson<CommandTypesJSON> for CommandTypes {
    fn to_json(&self, _: &super::Connection) -> anyhow::Result<CommandTypesJSON> {
        match self {
            CommandTypes::Sell => Ok(CommandTypesJSON::Sell),
            CommandTypes::SellIfHigh => Ok(CommandTypesJSON::SellIfHigh),
            CommandTypes::SellIfLow => Ok(CommandTypesJSON::SellIfLow),
            CommandTypes::BuyIfLow => Ok(CommandTypesJSON::BuyIfLow),
        }
    }
}

impl FromJson<CommandTypesJSON> for CommandTypes {
    fn from_json(json: &CommandTypesJSON) -> Self {
        match json {
            CommandTypesJSON::Sell => CommandTypes::Sell,
            CommandTypesJSON::SellIfHigh => CommandTypes::SellIfHigh,
            CommandTypesJSON::SellIfLow => CommandTypes::SellIfLow,
            CommandTypesJSON::BuyIfLow => CommandTypes::BuyIfLow,
        }
    }
}

#[derive(Queryable, Serialize, Deserialize,Clone, Associations, Identifiable)]
#[table_name = "command"]
pub struct Command {
    pub id: i32,
    pub stonker_id: i32,
    pub company_id: i32,
    pub threshold: i32,
    pub share: i32, // eg.: 50% = 50 * 10000 = 500000
    pub kind: CommandTypes,
    #[serde(with = "ts_seconds")]
    pub created_at: chrono::NaiveDateTime,
}

// TODO: implement toJson for Command

#[derive(Insertable, Serialize, Deserialize, Clone)]
#[table_name = "command"]
pub struct NewCommand {
    pub stonker_id: i32,
    pub company_id: i32,
    pub threshold: i32,
    pub share: i32,
    pub kind: CommandTypes,
    pub created_at: chrono::NaiveDateTime,
}

impl FromJson<NewCommand> for NewCommand {
    fn from_json(json: &NewCommand) -> Self {
        NewCommand {
            stonker_id: json.stonker_id,
            company_id: json.company_id,
            threshold: json.threshold,
            share: json.share,
            kind: json.kind.clone(),
            created_at: Utc::now().naive_utc()
        }
    }
}

// TODO: implement fromJson for newCommand
