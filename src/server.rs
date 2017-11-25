
use iron::prelude::*;
use iron::{BeforeMiddleware, AfterMiddleware, typemap};
use time::precise_time_ns;
use std::path::Path;

use iron::Iron;
use staticfile::Static;
use mount::Mount;
use hello_world;
use ResponseTime;
use configuration::*;
use iron;
use rustix_bl;


pub fn build_server() -> iron::Listening {
    let mut chain = Chain::new(hello_world);
    chain.link_before(ResponseTime);
    chain.link_after(ResponseTime);
    let mut serv = Iron::new(chain).http("localhost:8080").unwrap();
    return serv;
}



pub fn execute_cervisia_server(with_config: &ServerConfig,
                               old_backend : Option<rustix_bl::rustix_backend::RustixBackend<rustix_bl::persistencer::TransientPersister>>,
                               old_server : Option<iron::Listening>) -> (rustix_bl::rustix_backend::RustixBackend<rustix_bl::persistencer::TransientPersister>, iron::Listening) {
    if old_server.is_some() {
        old_server.unwrap().close();
    };
    if old_backend.is_some() {
        let backe = old_backend.unwrap();
        //TODO: shutdown of old backend?
        println!("Shutting down old backend {:?}", backe);
    };


    let mut backend = rustix_bl::build_transient_backend();
    let mut server = build_server();


    return (backend, server);
}