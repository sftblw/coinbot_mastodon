extern crate mammut;
extern crate separator;

use crate::upbit;
use self::separator::Separatable;
use std::time::Duration;

pub async fn post_recent(coins: &[String], mstdn: &mammut::Mastodon) -> anyhow::Result<()> {
    let mut content = String::new();

    content.push_str("upbit 암호화폐 KRW 시세\n");
    for coin in coins.into_iter() {

        let coin_result = get_recent_by_coin(coin).await;

        content.push_str(
            &format!("{:>6} : {} KRW\n",
            coin.to_string(),
            coin_result.separated_string()
        ));

        // TODO: remove this
        tokio::time::sleep(Duration::from_millis(500)).await;
    }

    let status = mammut::StatusBuilder{
        status: content,
        visibility: Some(mammut::status_builder::Visibility::Unlisted),
        ..Default::default()
    };

    let _result = mstdn.new_status(status);

    Ok(())
}

async fn get_recent_by_coin(coin: &String) -> f64 {
    let api_req = &(upbit::ApiRequest {
        period_type: upbit::PeriodType::Minutes,
        period: 60,
        market: upbit::Market::KRW,
        coin: coin.clone(),
        data_count: 1,
    });

    let upbit_result = upbit::request(
        &api_req
    ).await.unwrap();
    let upbit_data = upbit_result.get(0).unwrap();

    upbit_data.tradePrice
}