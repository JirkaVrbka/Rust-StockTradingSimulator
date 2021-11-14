use crate::schema::stonker;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Clone, Associations, Identifiable, PartialEq)]
#[table_name = "stonker"]
pub struct Stonker {
    pub id: i32,
    pub name: String,
    pub balance: i32,
}

#[derive(Insertable, Serialize, Deserialize, Clone)]
#[table_name = "stonker"]
pub struct NewStonker {
    pub name: String,
    pub balance: i32,
}
