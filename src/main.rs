extern crate coinbot_mastodon;
extern crate mammut;

use coinbot_mastodon::upbit;
use coinbot_mastodon::mastodon;
use coinbot_mastodon::post;

use std::path::Path;

fn main() {
    let mstdn = mastodon::load_or_auth("coinbot",
                                       "https://twingyeo.kr",
                                       &Path::new("./config/mastodon.toml"));
    execute(&mstdn);
//    mstdn.new_status(mammut::StatusBuilder::new("Hello, mammut! from disc-saved config".to_owned()));
}

fn execute(mstdn: &mammut::Mastodon) {
    let coins = [
        upbit::Coin::BTC,
        upbit::Coin::ETH,
        upbit::Coin::XRP,
        upbit::Coin::STEEM,
        upbit::Coin::SBD,
        upbit::Coin::ADA,
        upbit::Coin::QTUM,
        upbit::Coin::XMR,
    ];
    post::post_recent(&coins, &mstdn);
}
