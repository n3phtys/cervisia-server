
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
use reqwest;
use std::io::Read;


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




pub fn blocking_http_get_call(url: &str) -> Result<String, reqwest::Error> {

    let mut res = reqwest::get(url)?;

    println!("Status: {}", res.status());
    println!("Headers:\n{}", res.headers());

    let mut s : String = "".to_string();
    let size = res.read_to_string(&mut s);

    println!("Body:\n{}", s);

    println!("\n\nDone.");
    return Ok(s);
}





#[cfg(test)]
mod tests {



    use iron::Iron;
    use manager::*;
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
    use reqwest;
    use std::io::Read;
    use server::*;
    use manager::tests::fill_backend_with_medium_test_data;


    const HOST_PLUS_PORT : &'static str = "http://localhost:8081";

    fn get_server_config() -> ServerConfig {

        let port_str : String = HOST_PLUS_PORT.chars().skip("http://localhost:".len()).collect();
        return ServerConfig {

        use_send_mail: false,
        email_server: String::new(),
        email_username: String::new(),
        email_password: String::new(),
        top_items_per_user: 4,
        server_port: port_str.parse::<u16>().unwrap(),
    };}

    fn build_default_server() -> (RwLock<rustix_bl::rustix_backend::RustixBackend<rustix_bl::persistencer::TransientPersister>>, iron::Listening) {
        let default_server_conf = get_server_config();
        return execute_cervisia_server(&default_server_conf, None, None);
    }


    #[test]
    fn it_works() {
        assert!(1+1 == 2);
    }

    #[test]
    fn hello_world_works() {

        let (backend, server) = build_default_server();

        let httpbody = blocking_http_get_call(HOST_PLUS_PORT).unwrap();

        let mut server = server;
        server.close().unwrap();

        assert_eq!(httpbody, "Hello World");

    }

    #[test]
    fn second_hello_world_works() {

        let (backend, server) = build_default_server();

        let httpbody = blocking_http_get_call(HOST_PLUS_PORT).unwrap();

        let mut server = server;
        server.close().unwrap();

        assert_eq!(httpbody, "Hello World");

    }

    #[test]
    fn getting_all_users_works() {

        let (backend, server) = build_default_server();
        let mut server = server;
        fill_backend_with_medium_test_data(&backend);

        let params = ParametersAllUsers {
            count_pars: ParametersAllUsersCount {
                searchterm: "".to_string(),
            },
            pagination: ParametersPagination {
                start_inclusive: 0,
                end_exclusive: 1_000_000,
            },
        };
        let query = serde_json::to_string(&params).unwrap();
        let url = format!("{}/api/users/all?query={}", HOST_PLUS_PORT, query);

        let httpbody = blocking_http_get_call(&url).unwrap();

        server.close().unwrap();

        assert_eq!(httpbody, "Hello World");

    }
}