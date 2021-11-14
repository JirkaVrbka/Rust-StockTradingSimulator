use serde::{Deserialize, Serialize};
use crate::schema::stonker;

#[derive(Queryable, Serialize, Deserialize, Clone)]
pub struct Stonker {
    pub id: i32,
    pub name: String,
    pub balance: i32,
}

#[derive(Insertable, Serialize, Deserialize, Clone)]
#[table_name="stonker"]
pub struct NewStonker {
    pub name: String,
    pub balance: i32,
}
