extern crate coinbot_mastodon;
extern crate mammut;

use coinbot_mastodon::{coin_storage};
use coinbot_mastodon::mastodon;
use coinbot_mastodon::post;

use std::path::Path;
use coinbot_mastodon::coin_storage::CoinConfig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mstdn = mastodon::load_or_auth("coinbot",
                                       &Path::new("./config/mastodon.toml"));
    let coin_conf: CoinConfig = coin_storage::load_coins(&Path::new("./config/coins.toml"));

    post::post_recent(&coin_conf.coins, &mstdn).await?;

    Ok(())
}
