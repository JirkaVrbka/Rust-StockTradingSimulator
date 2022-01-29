pub mod home;
pub mod news;
pub mod search;
pub mod chat;
pub mod login;
pub mod logged;

pub use self::{home::Home, news::News, login::Login, logged::Logged, search::Search, chat::Chat};
