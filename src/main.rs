#[macro_use] extern crate rocket;
use rocket::serde::{Serialize, json::Json};
use scraper::{Html, Selector};
use reqwest;

// https://finance.yahoo.com/crypto
// https://finance.yahoo.com/robots.txt

// https://docs.rs/scraper/latest/scraper/

// https://docs.rs/reqwest/latest/reqwest/

// https://rocket.rs/v0.5-rc/guide/requests/

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
struct Cryptocurrency {
    symbol_name: String,
    name: String,
    price: f32,
    change: String,
    percent_change: String,
    market_cap: String,
    volume_in_currency_0_utc: String,
    volume_in_currency_24h: String,
    total_volume_all_currency: String,
    circulating_supply: String,
}

#[get("/")]
fn index() -> &'static str {
    "Yahoo Finance - Rust"
}

async fn crypto_handler() -> Result<Vec<Cryptocurrency>, reqwest::Error> {
    let body = reqwest::get("https://finance.yahoo.com/crypto").await?.text().await?;
    let document = Html::parse_document(&body);
    let tbody_selector = Selector::parse("tbody > tr").unwrap();
    let mut res: Vec<Cryptocurrency> = Vec::new();
    for tr in document.select(&tbody_selector) {
        let tr_vec = tr.text().collect::<Vec<_>>();
        // print!("{:?}", test);
        res.push(Cryptocurrency { 
            symbol_name: tr_vec[0].to_string(),
            name: tr_vec[1].to_string(), 
            price: tr_vec[2].trim().replace(',', "").parse::<f32>().unwrap(), 
            change: tr_vec[3].to_string(), 
            percent_change: tr_vec[4].to_string(), 
            market_cap: tr_vec[5].to_string(), 
            volume_in_currency_0_utc: tr_vec[6].to_string(), 
            volume_in_currency_24h: tr_vec[7].to_string(), 
            total_volume_all_currency: tr_vec[8].to_string(), 
            circulating_supply: tr_vec[9].to_string() });
    }
    Ok(res)
}

// -> Json<Cryptocurrency>

#[get("/crypto")]
async fn crypto() -> Json<Vec<Cryptocurrency>> {
    let res = crypto_handler().await.unwrap();
    Json(res)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, crypto])
}