extern crate chrono;
extern crate config;
extern crate iron;
#[macro_use]
extern crate lazy_static;
extern crate lettre;
extern crate lettre_email;
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
extern crate router;
extern crate rustix_bl;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate simple_logger;
extern crate staticfile;
extern crate time;
extern crate toml;
extern crate typescriptify;
#[macro_use]
extern crate typescriptify_derive;
extern crate uuid;
extern crate zip;

use configuration::*;
use iron::{AfterMiddleware, BeforeMiddleware, typemap};
use iron::Iron;
use iron::prelude::*;
use server::*;
use time::precise_time_ns;

pub mod mail;

pub mod configuration;

pub mod manager;

pub mod server;

pub mod billformatter;

pub mod importer;


pub struct ResponseTime;

impl typemap::Key for ResponseTime { type Value = u64; }

impl BeforeMiddleware for ResponseTime {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<ResponseTime>(precise_time_ns());
        Ok(())
    }
}

impl AfterMiddleware for ResponseTime {
    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
        let delta = precise_time_ns() - *req.extensions.get::<ResponseTime>().unwrap();
        println!("Request took: {} ms", (delta as f64) / 1000000.0);
        Ok(res)
    }
}

pub fn hello_world(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((iron::status::Ok, "Hello World")))
}


fn main() {
    simple_logger::init().unwrap();
    let config = ServerConfig::from_env();

    println!("Found following config: {:?}", &config);

    let listener = execute_cervisia_server(&config, None, None);
}
