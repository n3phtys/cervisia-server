
#[macro_use]
extern crate lazy_static;
extern crate iron;
extern crate staticfile;
extern crate time;
extern crate rustix_bl;
extern crate lettre_email;
#[macro_use]
extern crate log;
extern crate simple_logger;
extern crate mount;
extern crate lettre;
extern crate native_tls;
extern crate uuid;
extern crate notify;
extern crate config;
extern crate toml;
extern crate serde;
extern crate chrono;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate reqwest;
extern crate rand;
extern crate params;
extern crate router;
extern crate persistent;
#[macro_use]
extern crate typescriptify_derive;
extern crate typescriptify;

use iron::prelude::*;
use iron::{BeforeMiddleware, AfterMiddleware, typemap};
use time::precise_time_ns;

use iron::Iron;
use server::*;
use configuration::*;

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
    let logger = simple_logger::init().unwrap();

    let path = path_to_config_file_and_mkdirs();
    watch_config_changes(&path, execute_cervisia_server);
}
