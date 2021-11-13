use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Clone)]
pub struct Stonker {
    pub id: i32,
    pub name: String,
    pub balance: i32,
}
