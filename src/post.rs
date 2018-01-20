extern crate mammut;

use upbit;

use std::{thread, time};

pub fn post_recent(coins: &[upbit::Coin], mstdn: &mammut::Mastodon) {
    let mut content = String::new();
    content.push_str("upbit 암호화폐 KRW 시세\n");
    for coin in coins.into_iter() {
//        println!("coin: {}", coin.to_string());
        content.push_str(&format!("{:>6} : {} KRW\n"
                                  ,coin.to_string()
                                  ,readable( get_recent_by_coin(&coin).to_string() )
        ));
        thread::sleep(time::Duration::from_millis(500));
    }

    let status = mammut::StatusBuilder{
        status: content,
        visibility: Some(mammut::status_builder::Visibility::Unlisted),
        ..Default::default() // 이거 처음보는데 좋아보인다
    };

    let result = mstdn.new_status(status);
    // mammut 버그로 보임. // Serde(ErrorImpl { code: Message("missing field `error`"), line: 1, column: 2335 })
//    if let Err(e) = result {
//        panic!("error happend while posting to mastodon: {:?}", e);
//    }
}

fn get_recent_by_coin(coin: &upbit::Coin) -> f64 {
//    println!("coin: {}", coin.to_string());
    let api_req = &(upbit::ApiRequest {
        period_type: upbit::PeriodType::Minutes,
        period: 60,
        market: upbit::Market::KRW,
        coin: coin.clone(),
        data_count: 1,
    });
    let upbit_result = upbit::request(
        &api_req
    ).unwrap();
    let upbit_data = upbit_result.get(0).unwrap();

    upbit_data.tradePrice
}

/// format to readable with comma and minus
/// [copied from S/O](https://stackoverflow.com/a/34714201)
fn readable(mut o_s: String) -> String {
    let mut s = String::new();
    let mut negative = false;
    let values: Vec<char> = o_s.chars().collect();
    if values[0] == '-' {
        o_s.remove(0);
        negative = true;
    }
    for (i ,char) in o_s.chars().rev().enumerate() {
        if i % 3 == 0 && i != 0 {
            s.insert(0, ',');
        }
        s.insert(0, char);
    }
    if negative {
        s.insert(0, '-');
    }
    return s
}