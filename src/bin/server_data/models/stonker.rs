use crate::schema::stonker;
use serde::{Deserialize, Serialize};

use super::command::CommandTypes;

#[derive(Queryable, Serialize, Deserialize, Clone, Associations, Identifiable, PartialEq)]
#[table_name = "stonker"]
pub struct Stonker {
    pub id: i32,
    pub name: String,
    pub balance: i32,
    pub blocked_balance: i32,
    pub invested_balance: i32,
}

#[derive(Insertable, Serialize, Deserialize, Clone)]
#[table_name = "stonker"]
pub struct NewStonker {
    pub name: String,
    pub balance: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct StonkerOverviewJSON {
    pub portfolio: Vec<PortfolioJSON>,
    pub usage: UsageJSON,
    // TODO: pub graph: unknown_yet,
    pub stonker_history: Vec<StonkerHistoryJSON>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PortfolioJSON {
    pub stock: String,
    pub share: i32,
    pub money: i32,
    pub difference: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UsageJSON {
    pub free: i32,
    pub invested: i32,
    pub blocked: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct StonkerHistoryJSON {
    pub day: String,
    pub action: CommandTypes,
    pub stock: String,
    pub money: i32,
}
