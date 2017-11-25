extern crate iron;
extern crate staticfile;
extern crate time;
extern crate rustix_bl;
#[macro_use]
extern crate log;
extern crate simple_logger;
extern crate mount;
extern crate lettre;
extern crate notify;
extern crate config;
extern crate toml;
#[macro_use]
extern crate serde_derive;



use iron::prelude::*;
use iron::{BeforeMiddleware, AfterMiddleware, typemap};
use time::precise_time_ns;
use std::path::Path;

use iron::Iron;
use staticfile::Static;
use mount::Mount;

pub mod mail;

pub mod configuration;

pub mod manager;

pub mod server;


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

fn timer_server() {
    let mut chain = Chain::new(hello_world);
    chain.link_before(ResponseTime);
    chain.link_after(ResponseTime);
    println!("Iron Builder Thread starts here");
    let mut serv = Iron::new(chain).http("localhost:8080").unwrap();

    println!("Iron Builder Thread now here");
    serv.close();
}




fn main() {
    simple_logger::init().unwrap();
    println!("Hello, world!");
    info!("Logging example message to info");
    timer_server();
}










//
//fn serve_static_files_to_root(dir: std::path::PathBuf) {
//    let mut mount = Mount::new();
//
//    // Serve the shared JS/CSS at /
//    mount.mount("/", Static::new(Path::new("target/doc/")));
//    // Serve the static file docs at /doc/
//    mount.mount("/doc/", Static::new(Path::new("target/doc/staticfile/")));
//    // Serve the source code at /src/
//    mount.mount("/src/", Static::new(Path::new("target/doc/src/staticfile/lib.rs.html")));
//
//    println!("Doc server running on http://localhost:3000/doc/");
//
//    Iron::new(mount).http("127.0.0.1:3000").unwrap();
//}

