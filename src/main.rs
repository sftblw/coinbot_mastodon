extern crate coinbot_mastodon;
extern crate mammut;

use coinbot_mastodon::{coin_storage};
use coinbot_mastodon::mastodon;
use coinbot_mastodon::post;

use std::path::Path;
use coinbot_mastodon::coin_storage::CoinConfig;

fn main() {
    let mstdn = mastodon::load_or_auth("coinbot",
                                       &Path::new("./config/mastodon.toml"));
    let coin_conf: CoinConfig = coin_storage::load_coins(&Path::new("./config/coins.toml"));
    execute(&mstdn, coin_conf.coins);
}

fn execute(mstdn: &mammut::Mastodon, coins: Vec<String>) {
    post::post_recent(&coins, &mstdn);
}
