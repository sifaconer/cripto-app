use std::io::stdin;
use serde::{Serialize, Deserialize};
use serde_json::{from_str};
use rusty_money::{Money, iso};

fn main() {
    let mut coin: String = String::new();

    println!("Nombre de la Criptomoneda a connsultar: ");
    let _ = stdin().read_line(&mut coin).expect("ERROR");
    match get_precio(&coin) {
        Ok(coin_data) => {
            println!("-===================================-");
            println!("Para: {}", coin_data.name);
            println!("ID: {}", coin_data.id);
            println!("Image: {}", coin_data.image.small);
            let usd = Money::from_str(
                    &coin_data.market_data.current_price.usd.to_string(),
                    iso::USD)
            .unwrap();
            println!("Precio en USD: {}", usd);
        }
        Err(error)=>{
            println!("ERROR: {}", error)
        }
    }
}

#[derive(Serialize, Deserialize)]
struct CoinData {
    id: String,
    name: String,
    image: Image,
    market_data: MarketData
}

#[derive(Serialize, Deserialize)]
struct Image {
    thumb: String,
    small: String,
    large: String
}

#[derive(Serialize, Deserialize)]
struct MarketData {
    current_price: CurretPrice
}

#[derive(Serialize, Deserialize)]
struct CurretPrice {
    usd: f32
}

fn get_precio(coin: &str) -> Result<CoinData, ureq::Error>  {
    let path: String = format!("https://api.coingecko.com/api/v3/coins/{}?localization=false&developer_data=false", coin);

    let body: String = ureq::get(&path)
    .set("accept", "application/json")
    .call()?.into_string()?;

    let coin_data: CoinData = from_str(&body).unwrap();
    Ok(coin_data)
}

