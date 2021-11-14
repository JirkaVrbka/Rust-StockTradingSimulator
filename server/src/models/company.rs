use serde::{Deserialize, Serialize};
use crate::schema::company;


#[derive(Queryable, Serialize, Deserialize, Clone, Associations, Identifiable, PartialEq)]
pub struct Company {
    pub id: i32,
    pub name: String,
    pub performer_id: i32,
}

#[derive(Insertable, Serialize, Deserialize, Clone)]
#[table_name="company"]
pub struct NewCompany {
    pub name: String,
    pub performer_id: i32,
}
