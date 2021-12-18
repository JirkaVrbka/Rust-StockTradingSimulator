use crate::schema::command;
use chrono::naive::serde::ts_seconds;
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
use utils::json::CommandTypesJSON;

use super::ToJson;

#[derive(Serialize, Deserialize, Clone, DbEnum, Debug)]
#[DieselType = "Commandtypesdb"]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CommandTypes {
    SELL,
    SELL_IF_HIGH,
    SELL_IF_LOW,
    BUY_IF_LOW,
}

impl ToJson<CommandTypesJSON> for CommandTypes {
    fn to_json(&self, _: &super::Connection) -> anyhow::Result<CommandTypesJSON> {
        match self {
            CommandTypes::SELL => Ok(CommandTypesJSON::SELL),
            CommandTypes::SELL_IF_HIGH => Ok(CommandTypesJSON::SELL_IF_HIGH),
            CommandTypes::SELL_IF_LOW => Ok(CommandTypesJSON::SELL_IF_LOW),
            CommandTypes::BUY_IF_LOW => Ok(CommandTypesJSON::BUY_IF_LOW),
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
