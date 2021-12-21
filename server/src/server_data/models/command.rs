use crate::schema::command;
use chrono::naive::serde::ts_seconds;
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
use utils::json::CommandTypesJSON;

use super::ConvertJson;

#[derive(Serialize, Deserialize, Clone, DbEnum, Debug)]
#[DieselType = "Commandtypesdb"]
#[DbValueStyle = "SCREAMING_SNAKE_CASE"]
pub enum CommandTypes {
    Sell,
    SellIfHigh,
    SellIfLow,
    BuyIfLow,
}

impl ConvertJson<CommandTypesJSON> for CommandTypes {
    fn to_json(&self, _: &super::Connection) -> anyhow::Result<CommandTypesJSON> {
        match self {
            CommandTypes::Sell => Ok(CommandTypesJSON::Sell),
            CommandTypes::SellIfHigh => Ok(CommandTypesJSON::SellIfHigh),
            CommandTypes::SellIfLow => Ok(CommandTypesJSON::SellIfLow),
            CommandTypes::BuyIfLow => Ok(CommandTypesJSON::BuyIfLow),
        }
    }
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

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "command"]
pub struct NewCommand {
    pub stonker_id: i32,
    pub company_id: i32,
    pub threshold: i32,
    pub share: i32,
    pub kind: CommandTypes,
}
