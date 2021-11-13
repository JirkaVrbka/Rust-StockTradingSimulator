use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Clone)]
pub struct Company {
    pub id: i32,
    pub name: String,
    pub performer_id: i32,
}
