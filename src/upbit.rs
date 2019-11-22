extern crate reqwest;
extern crate serde_json;

use std::error::Error;

pub fn request(req: &ApiRequest) -> Result<Vec<ApiResult>, Box<Error>>{
    let url = req.to_endpoint();
//    println!("url: {}", url);
    let mut result = reqwest::get(&url)?;

    // I don't know why but parsing json should be done in separated...
    let text = result.text().unwrap();
//    println!("result: {}", &text);

    let result: Vec<ApiResult> = serde_json::from_str(&text)?;

    Ok(result)
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct ApiResult {
    pub code: String,
    pub candleDateTime: String,
    pub candleDateTimeKst: String,
    pub openingPrice: f64,
    pub highPrice: f64,
    pub lowPrice: f64,
    pub tradePrice: f64,
    pub candleAccTradeVolume: f64,
    pub candleAccTradePrice: f64,
    pub timestamp: u64,
    pub unit: u64,
}

#[allow(non_snake_case)]
pub struct ApiRequest {
    pub period_type: PeriodType,
    pub period: u8,
    pub market: Market,
    pub coin: String,
    pub data_count: u8
}

impl ApiRequest {
    pub fn to_endpoint(&self) -> String {
        format!("https://crix-api-endpoint.upbit.com/v1/crix/candles/{}/{}?code=CRIX.UPBIT.{}-{}&count={}"
            ,self.period_type.to_string()
            ,self.period.to_string()
            ,self.market.to_string()
            ,self.coin.to_string()
            ,self.data_count.to_string()
        )
    }
}

#[allow(non_snake_case)]
pub enum PeriodType {
    Minutes, Days, Weeks, Months
}

impl ToString for PeriodType {
    fn to_string(&self) -> String {
        match self {
            &PeriodType::Minutes => "minutes".to_owned(),
            &PeriodType::Days => "days".to_owned(),
            &PeriodType::Weeks => "weeks".to_owned(),
            &PeriodType::Months => "months".to_owned(),
        }
    }
}

#[allow(non_snake_case)]
pub enum Market {
    KRW, BTC, ETH, USDTT
}

impl ToString for Market {
    fn to_string(&self) -> String {
        match self {
            &Market::KRW => "KRW".to_owned(),
            &Market::BTC => "BTC".to_owned(),
            &Market::ETH => "ETH".to_owned(),
            &Market::USDTT => "USDTT".to_owned(),
        }
    }
}

#[derive(Clone, Copy)]
#[allow(non_snake_case)]
pub enum Coin {
    BTC, ETH, XRP, STEEM, SBD, ADA, QTUM, XMR
}

impl ToString for Coin {
    fn to_string(&self) -> String {
        match self {
            &Coin::BTC => "BTC".to_owned(),
            &Coin::ETH => "ETH".to_owned(),
            &Coin::XRP => "XRP".to_owned(),
            &Coin::STEEM => "STEEM".to_owned(),
            &Coin::SBD => "SBD".to_owned(),
            &Coin::ADA => "ADA".to_owned(),
            &Coin::QTUM => "QTUM".to_owned(),
            &Coin::XMR => "XMR".to_owned(),
        }
    }
}
