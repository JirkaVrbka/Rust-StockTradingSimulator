use diesel::sql_types::Timestamp;

#[derive(Queryable)]
pub struct History {
    pub id: i32,
    pub stonker_id: i32,
    pub stock_id: i32,
    pub bought_for: i32,
    pub created_at: Timestamp,
    pub sold_for: i32,
}
