
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


pub fn build_server(port: u16) -> iron::Listening {
    let mut chain = Chain::new(hello_world);
    chain.link_before(ResponseTime);
    chain.link_after(ResponseTime);
    let mut serv = Iron::new(chain).http(format!("localhost:{}", port)).unwrap();
    return serv;
}



pub fn execute_cervisia_server(with_config: &ServerConfig,
                               old_backend : Option<rustix_bl::rustix_backend::RustixBackend<rustix_bl::persistencer::TransientPersister>>,
                               old_server : Option<iron::Listening>) -> (rustix_bl::rustix_backend::RustixBackend<rustix_bl::persistencer::TransientPersister>, iron::Listening) {

    info!("execute_cervisia_server begins for config = {:?}", with_config);

    if old_server.is_some() {
        info!("Closing old server");
        //TODO: does not work, see https://github.com/hyperium/hyper/issues/338
        old_server.unwrap().close().unwrap();
    };
    if old_backend.is_some() {
        info!("Closing old backend");
        let backe = old_backend.unwrap();
        //TODO: shutdown of old backend?
        println!("Shut down old backend {:?}", backe);
    };


    info!("Building backend");

    let mut backend = rustix_bl::build_transient_backend();


    info!("Building server");

    let mut server = build_server(with_config.server_port);

    println!("Having built server");


    return (backend, server);
}