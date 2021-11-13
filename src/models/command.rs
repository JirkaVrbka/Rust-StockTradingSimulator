pub enum CommandTypes {
    SELL, SELL_IF_HIGH, SELL_IF_LOW, BUY_IF_LOW
}

#[derive(Queryable)]
pub struct Command {
    pub id: i32,
    stonker_id: i32,
    company_id: i32,
    threshold: i32,
    share: f32,
    r#type: CommandTypes
}

