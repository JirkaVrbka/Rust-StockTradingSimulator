use serde::{Deserialize, Serialize};
use crate::schema::stock;

#[derive(Queryable, Serialize, Deserialize, Clone, Associations, Identifiable, PartialEq)]
pub struct Stock {
    pub id: i32,
    pub stonker_id: i32,
    pub company_id: i32,
}

#[derive(Insertable, Serialize, Deserialize, Clone)]
#[table_name="stock"]
pub struct NewStock {
    pub stonker_id: i32,
    pub company_id: i32,
}