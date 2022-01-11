use chrono::{NaiveDateTime, NaiveDate};

use crate::{json::{CommandJSON, CommandTypesJSON}, datagen::ToTSQLValue};

use super::{Generator, IndexVec, ToTSQL, Data, TSQLValue, JsonGenerator};

impl ToTSQLValue for CommandTypesJSON {
    fn to(&self) -> TSQLValue {
        match self {
            CommandTypesJSON::Sell => "SELL",
            CommandTypesJSON::SellIfHigh => "SELL_IF_HIGH",
            CommandTypesJSON::SellIfLow => "SELL_IF_LOW",
            CommandTypesJSON::BuyIfLow => "BUY_IF_LOW",
        }.to_string().to()
    }
}

impl ToTSQL for CommandJSON {
    fn to_header() -> &'static str {
        "Command"
    }
    fn to_columns() -> Vec<&'static str> {
        vec!["id", "stonker_id", "company_id", "threshold", "share", "kind", "created_at"]
    }
    fn to_data(&self) -> Vec<TSQLValue> {
        vec![self.id.to_id(), self.stonker.id.to_id(), self.company.id.to_id(),
            self.threshold.to(), self.share.to(), self.kind.to(), self.created_at.to()]
    }
}

pub struct CommandGenerator;

impl JsonGenerator for CommandGenerator {
    fn new() -> anyhow::Result<CommandGenerator> {
        Ok(CommandGenerator)
    }
    fn create(&mut self, generator: &mut Generator, data: &mut Data) {
        // choose random stonker, if he owns stocks, put random sell command on their random stock
        // if he does not own stocks, put buy_if_low command on random stock
    }
}