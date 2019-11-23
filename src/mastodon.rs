extern crate mammut;
extern crate toml;

use self::mammut::{Registration, Data};
use self::mammut::apps::{AppBuilder, Scopes};

use std::path::Path;
use std::fs::File;
use std::io;
use std::io::prelude::*;

use std::error::Error;
use util;

pub fn load_or_auth(app_name: &str, store_path: &Path) -> mammut::Mastodon {
    // try load and if fails do the auth process.
    let mammut_data: Data = match {
        |store_path: &Path| -> Result<mammut::Data, Box<dyn Error>>{
            let mut buf = String::new();
            File::open(store_path)?.read_to_string(&mut buf)?;
            let data: mammut::Data = toml::from_str(&buf)?;

            Ok(data)
        }(store_path)
    } {
        Ok(d) => d,
        Err(_) => {
            let data = auth(app_name)
                .expect("Mastodon auth process somehow failed.");

            let p = Path::new(store_path);
            util::create_dir_by_ext(p).unwrap();

            File::create(store_path)
                .expect(&format!("can't create mastodon config file {}",
                                store_path.to_str().unwrap())
                ).write_all(
                    &toml::to_string(&data)
                        .expect("can't serialize mastodon config!").into_bytes()
                ).expect(&format!("can't write mastodon config file {}",
                                 store_path.to_str().unwrap())
                );

            data
        },
    };

    mammut::Mastodon::from_data(mammut_data)
}

fn auth(app_name: &str) -> Result<mammut::Data, Box<dyn Error>> {
    let app = AppBuilder {
        client_name: app_name,
        redirect_uris: "urn:ietf:wg:oauth:2.0:oob",
        scopes: Scopes::Write,
        website: None,
    };

    println!("# Mastodon authorization process");
    println!();

    print!("# instance: ");
    io::stdout().flush()?;
    let mut instance_url = String::new();
    io::stdin().read_line(&mut instance_url)?;
    instance_url = instance_url.trim_end().to_owned();

    let mut registration = Registration::new(instance_url);
    registration.register(app)?;
    let url = registration.authorise()?;

    println!("Open this url \" {} \"", url);
    println!("and Authrorize, and paste key and press enter.");
    print!("key :");
    io::stdout().flush()?;

    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;

    let code: String = buf.trim().to_owned();
    let mastodon = registration.create_access_token(code)?;

    Ok(mastodon.data)
}