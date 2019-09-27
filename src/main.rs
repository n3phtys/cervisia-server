extern crate chrono;
extern crate config;
extern crate iron;
extern crate lettre;
extern crate lettre_email;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate mime;
extern crate mount;
extern crate native_tls;
extern crate notify;
extern crate params;
extern crate persistent;
extern crate rand;
extern crate reqwest;
extern crate url;
extern crate router;
extern crate rustix_bl;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate env_logger;
extern crate serde_json;
extern crate staticfile;
extern crate time;
extern crate toml;
extern crate typescriptify;
#[macro_use]
extern crate typescriptify_derive;
extern crate openssl_probe;
extern crate uuid;
extern crate zip;
extern crate jsonwebtoken as jwt;

use configuration::*;
use server::*;
use std::sync::Mutex;

pub mod mail;

pub mod configuration;

pub mod manager;

pub mod server;

pub mod billformatter;

pub mod importer;

lazy_static! {
    static ref HELLO_SYSTEM: Mutex<String> = Mutex::new("Hello System".to_string());
}

fn main() {
    println!("{}", HELLO_SYSTEM.lock().unwrap());

    openssl_probe::init_ssl_cert_env_vars();
    env_logger::init();

    let config = ServerConfig::from_env();

    info!("Found following config: {:?}", &config);

    let _listener = execute_cervisia_server(&config, None, None);
}
