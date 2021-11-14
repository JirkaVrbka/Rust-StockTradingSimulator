use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
use crate::schema::command;

#[derive(Serialize, Deserialize, Clone, DbEnum, Debug)]
#[DieselType = "Commandtypes"]
pub enum CommandTypes {
    SELL,
    SELL_IF_HIGH,
    SELL_IF_LOW,
    BUY_IF_LOW,
}

#[derive(Queryable, Serialize, Deserialize, Clone, Associations, Identifiable)]
#[table_name="command"]
pub struct Command {
    pub id: i32,
    pub stonker_id: i32,
    pub company_id: i32,
    pub threshold: i32,
    pub share: i32, // eg.: 50% = 50 * 10000 = 500000
    pub kind: CommandTypes,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name="command"]
pub struct NewCommand {
    pub stonker_id: i32,
    pub company_id: i32,
    pub threshold: i32,
    pub share: i32,
    pub kind: CommandTypes,
}
