
use iron::prelude::*;
use iron::{BeforeMiddleware, AfterMiddleware, typemap};
use time::precise_time_ns;
use std::path::Path;
use std::collections::*;
use std::error::Error;

use iron::Iron;
use staticfile::Static;
use mount::Mount;
use hello_world;
use ResponseTime;
use configuration::*;
use iron;
use rustix_bl;
use serde_json;
use std;
use rustix_bl::rustix_event_shop;
use std::sync::RwLock;
use manager::ParametersAll;


pub fn build_server(port: u16) -> iron::Listening {
    let mut chain = Chain::new(hello_world);
    chain.link_before(ResponseTime);
    chain.link_after(ResponseTime);
    let mut serv = Iron::new(chain).http(format!("localhost:{}", port)).unwrap();
    return serv;
}



pub fn execute_cervisia_server(with_config: &ServerConfig,
                               old_backend : Option<RwLock<rustix_bl::rustix_backend::RustixBackend<rustix_bl::persistencer::TransientPersister>>>,
                               old_server : Option<iron::Listening>) -> (RwLock<rustix_bl::rustix_backend::RustixBackend<rustix_bl::persistencer::TransientPersister>>, iron::Listening) {

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

    let mut backend = RwLock::new(rustix_bl::build_transient_backend());


    info!("Building server");

    let mut server = build_server(with_config.server_port);

    println!("Having built server");


    return (backend, server);
}


pub struct ServerWriteResult {
    pub error_message : Option<String>,
    pub is_success : bool,
    pub content : Option<SuccessContent>,
}

pub struct SuccessContent {
    pub timestamp : u64,
    pub refreshed_data : HashMap<String, serde_json::Value>,
}

pub trait RefreshStateExt {
    fn from_request() -> Self;
}


impl RefreshStateExt for ParametersAll {
    //called for writes, as we need the full set of parameters
    fn from_request() -> Self {
        unimplemented!()
    }
}


pub trait WriteApplicator {

    type ErrorType : std::error::Error;

fn apply_write(backend : &mut rustix_bl::rustix_backend::RustixBackend<rustix_bl::persistencer::TransientPersister>, event : rustix_event_shop::BLEvents ) -> Result<SuccessContent, Self::ErrorType>;
    fn apply_write_to_result(backend : &mut rustix_bl::rustix_backend::RustixBackend<rustix_bl::persistencer::TransientPersister>, event : rustix_event_shop::BLEvents) -> ServerWriteResult {
        let r = Self::apply_write(backend,event);
        return match r {
            Ok(res) => ServerWriteResult {
                error_message: None,
                is_success: true,
                content: Some(res),
            },
            Err(e) => ServerWriteResult {
                error_message: Some(e.description().to_string()),
                is_success: false,
                content: None,
            },
        };
    }
}