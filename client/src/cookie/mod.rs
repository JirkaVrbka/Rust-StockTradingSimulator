use wasm_bindgen::JsCast;
use anyhow::{Context, anyhow};

fn get_cookies() -> anyhow::Result<String> {
    let window = web_sys::window().context("Can't find Window!")?;
    let document = window.document().context("Can't find Document!")?;
    let html_document = document.dyn_into::<web_sys::HtmlDocument>().map_err(|_| anyhow!("Can't find HTML Document!"))?;
    html_document.cookie().map_err(|_| anyhow!("Can't find Cookies!"))
}

pub fn is_logged() -> anyhow::Result<bool> {
    let cookies = get_cookies()?;
    Ok(cookies.contains("user_id") && cookies.contains("passwd"))
}

pub fn get_login() -> anyhow::Result<i32> {
    let cookies = get_cookies()?;
    let user_id = "user_id";
    let user = cookies.find(user_id).context("Can't find user_id in cookies")?;
    let id: String = cookies.chars().skip(user + user_id.len() + 1).take_while(|c| c.is_alphanumeric()).collect();
    id.parse::<i32>().map_err(|_| anyhow!("Parse Error"))
}
