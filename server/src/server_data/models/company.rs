use crate::schema::company;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Clone, Associations, Identifiable, PartialEq)]
#[table_name = "company"]
pub struct Company {
    pub id: i32,
    pub name: String,
    pub performer_id: i32,
}

#[derive(Insertable, Serialize, Deserialize, Clone)]
#[table_name = "company"]
pub struct NewCompany {
    pub name: String,
    pub performer_id: i32,
}
