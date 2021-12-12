use crate::models::company::Company;
use crate::models::stonker::Stonker;
use crate::schema::stock;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Clone, Associations, Identifiable, PartialEq)]
#[belongs_to(Company)]
#[belongs_to(Stonker)]
#[table_name = "stock"]
pub struct Stock {
    pub id: i32,
    pub stonker_id: i32,
    pub company_id: i32,
    pub share: i32, // eg.: 50% = 50 * 10000 = 500000
    pub bought_for: i32,
    pub sold_for: Option<i32>,
}

#[derive(Insertable, Serialize, Deserialize, Clone)]
#[table_name = "stock"]
pub struct NewStock {
    pub stonker_id: i32,
    pub company_id: i32,
    pub share: i32, // eg.: 50% = 50 * 10000 = 500000
    pub bought_for: i32
}
