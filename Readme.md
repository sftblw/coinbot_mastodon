# Coinbot: upbit.com → Mastodon (KRW)

Practice-purposed small app which posts crypto-currency price to mastodon instance.

Because This is a toy project, It will or will not extended or documented. I don't know about it.

binary project. "config" folder is required to run.

## in detail

1. Add this app to the mastodon account. (mastodon.rs)
    - If possible, load from file system.
    - else Request user to do that. and records to fs.
2. reads upbit.com with the [unofficial API](https://steemit.com/kr/@segyepark/api) (upbit.rs, post.rs)
3. form it (post.rs)
4. post to mastodon (post.rs)

As you expect, post.rs and main.rs is not organized well.

## main dependency

- [Aaronepower/Mammut](https://github.com/Aaronepower/Mammut) for posting to mastodon
- reqwest (upbit.com access)
- toml (save to fs, load from fs)

## example

will not be provided since being a toy project. but you can refer to files, `main.rs` & `post.rs`.