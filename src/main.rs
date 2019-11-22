extern crate coinbot_mastodon;
extern crate mammut;

use coinbot_mastodon::upbit;
use coinbot_mastodon::mastodon;
use coinbot_mastodon::post;

use std::path::Path;

fn main() {
    let mstdn = mastodon::load_or_auth("coinbot",
                                       &Path::new("./config/mastodon.toml"));
    execute(&mstdn);
//    mstdn.new_status(mammut::StatusBuilder::new("Hello, mammut! from disc-saved config".to_owned()));
}

fn execute(mstdn: &mammut::Mastodon) {
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
    post::post_recent(&coins, &mstdn);
}
