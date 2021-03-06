use serde::{Serialize, Deserialize};
use chrono::naive::serde::ts_seconds;
use serde_repr::{Serialize_repr, Deserialize_repr};
use strum::{EnumIter};

#[derive(Debug, Serialize, Deserialize, Clone, EnumIter)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CommandTypesJSON {
    Sell,
    SellIfHigh,
    SellIfLow,
    BuyIfLow,
}

#[derive(PartialEq, Eq, Hash, Debug, Serialize_repr, Deserialize_repr, Clone, EnumIter)]
#[repr(i8)]
pub enum EffectJSON {
    Fall = -1,
    Neutral = 0,
    Rise = 1,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewsJSON {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub author: String,
    #[serde(with = "ts_seconds")]
    pub created_at: chrono::NaiveDateTime,
    pub effect: EffectJSON,
    pub company: CompanyJSON,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StonkerJSON {
    pub id: i32,
    pub name: String,
    pub balance: i32,
    pub blocked_balance: i32,
    pub invested_balance: i32,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StonkerCredentials {
    pub name: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommandJSON {
    pub id: i32,
    pub stonker: StonkerJSON,
    pub company: CompanyJSON,
    pub threshold: i32,
    pub share: i32, // eg.: 50% = 50 * 10000 = 500000
    pub kind: CommandTypesJSON,
    #[serde(with = "ts_seconds")]
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommandCreateJson {
    pub stonker_id: i32,
    pub company_id: i32,
    pub threshold: i32,
    pub share: i32, // eg.: 50% = 50 * 10000 = 500000
    pub kind: i32
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StockJSON {
    pub id: i32,
    pub owner: StonkerJSON,
    pub issued_by: CompanyJSON,
    pub share: i32, // eg.: 50% = 50 * 10000 = 500000
    pub bought_for: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CompanyValueJSON {
    pub datetime: chrono::NaiveDateTime,
    pub value: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CompanyJSON {
    pub id: i32,
    pub name: String,
    pub performer: StonkerJSON,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CompanyDetailJSON {
    pub id: i32,
    pub name: String,
    pub performer: StonkerJSON,
    pub value_history: Vec<CompanyValueJSON>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CheapestStocksJSON {
    pub company: CompanyJSON,
    pub stock: StockJSON,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HistoryJSON {
    pub id: i32,
    pub owned_by: StonkerJSON,
    pub issued_by: CompanyJSON,
    pub bought_for: i32,
    #[serde(with = "ts_seconds")]
    pub created_at: chrono::NaiveDateTime,
    pub sold_for: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StonkerOverviewJSON {
    pub portfolio: Vec<PortfolioJSON>,
    pub usage: UsageJSON,
    pub portfolio_overview: Vec<PortfolioJSON>,
    pub stonker_history: Vec<StonkerHistoryJSON>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PortfolioJSON {
    pub stock: String,
    pub share: i32,
    pub money: i32,
    pub difference: i32,
    pub bought_for: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UsageJSON {
    pub free: i32,
    pub invested: i32,
    pub blocked: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StonkerHistoryJSON {
    pub day: String,
    pub action: CommandTypesJSON,
    pub stock: String,
    pub money: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthJSON {
    NoSuchUser,
    WrongPassword,
    Ok,
}