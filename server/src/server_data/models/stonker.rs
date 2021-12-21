use crate::schema::stonker;
use serde::{Deserialize, Serialize};
use utils::json::StonkerJSON;
use super::{Connection, ToJson, FromJson};

#[derive(Queryable, Serialize, Deserialize, Clone, Associations, Identifiable, PartialEq)]
#[table_name = "stonker"]
pub struct Stonker {
    pub id: i32,
    pub name: String,
    pub balance: i32,
    pub blocked_balance: i32,
    pub invested_balance: i32,
}

impl ToJson<StonkerJSON> for Stonker {
    fn to_json(
        &self,
        _connection: &Connection,
    ) -> anyhow::Result<StonkerJSON> {
        Ok(StonkerJSON {
            id: self.id,
            name: self.name.clone(),
            balance: self.balance,
            blocked_balance: self.blocked_balance,
            invested_balance: self.invested_balance,
        })
    }
}

#[derive(Insertable, Serialize, Deserialize, Clone)]
#[table_name = "stonker"]
pub struct NewStonker {
    pub name: String,
    pub balance: i32,
}

impl FromJson<StonkerJSON> for NewStonker {
    fn from_json(json: &StonkerJSON) -> Self {
        NewStonker {
            name: json.name.clone(),
            balance: json.balance,
        }
    }
}