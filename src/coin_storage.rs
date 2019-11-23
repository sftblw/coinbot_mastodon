extern crate toml;

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use util;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CoinConfig {
    pub coins: Vec<String>
}

impl CoinConfig {
    pub fn new(coins: Vec<String>) -> CoinConfig {
        CoinConfig { coins }
    }
}

pub fn load_coins(path: &Path) -> CoinConfig {
    let coins_data: CoinConfig = match {
        |path: &Path| -> Result<CoinConfig, Box<dyn Error>>{
            let mut buf = String::new();
            File::open(path)?.read_to_string(&mut buf)?;
            let config: CoinConfig = toml::from_str(&buf)?;

            Ok(config)
        }(path)
    } {
        Ok(d) => d,
        Err(_) => {
            let coins = coins_cli();

            let p = Path::new(path);

            util::create_dir_by_ext(p).unwrap();

            File::create(path)
                .expect(&format!("can't create coins config file {}",
                                 path.to_str().unwrap())
                ).write_all(
                &toml::to_string(&coins)
                    .expect("can't serialize coins config!").into_bytes()
            ).expect(&format!("can't write coins config file {}",
                              path.to_str().unwrap())
            );

            coins
        },
    };

    coins_data
}

fn coins_cli() -> CoinConfig {
    let coins = [
        "BTC".to_owned(),
        "ETH".to_owned(),
        "XRP".to_owned(),
        "STEEM".to_owned(),
        "SBD".to_owned(),
        "ADA".to_owned(),
        "QTUM".to_owned(),
        "XMR".to_owned()
    ];

    CoinConfig::new(coins.to_vec())
}