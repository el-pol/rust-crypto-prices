use serde::Deserialize;
use serde_json::Map;
use serde_json::Value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::Request;
use web_sys::RequestInit;

#[wasm_bindgen]
pub async fn get_coin_price(coin: String) -> Result<String, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(web_sys::RequestMode::Cors);

    let url = format!("https://api.coingecko.com/api/v3/coins/{coin}?localization=false&tickers=false&market_data=true&community_data=false&developer_data=false");
    let request = Request::new_with_str_and_init(&url, &opts).unwrap();
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .unwrap();
    let resp = resp_value.dyn_into::<web_sys::Response>().unwrap();
    let text = JsFuture::from(resp.text()?).await?.as_string().unwrap();

    let coin_data: CoinData = parse_body(&text);

    Ok(coin_data.market_data.current_price["usd"].to_string())
}

fn parse_body(body: &str) -> CoinData {
    serde_json::from_str(body).unwrap()
}

#[derive(Deserialize)]
struct CoinData {
    id: String,
    name: String,
    symbol: String,
    image: Map<String, Value>,
    market_data: MarketData,
}

#[derive(Deserialize)]
struct MarketData {
    current_price: Map<String, Value>,
}
