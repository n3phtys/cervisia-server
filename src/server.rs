
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
use std::sync::{Arc, RwLock};
use std::thread;
use manager::ParametersAll;
use reqwest;
use std::io::Read;
use iron::Handler;

use iron::prelude::*;
use iron::status;
use router::Router;
use responsehandlers::*;
use params;

use params::{Params, Value};

type Backend = rustix_bl::rustix_backend::RustixBackend<rustix_bl::persistencer::TransientPersister>;



pub fn build_server(backend : Arc<RwLock<Backend>>, port: u16) -> iron::Listening {
    let mut router = Router::new();

    let back2 = backend.clone();

    router.get("/users/all", move |req: &mut Request|{all_users(&back2, req)}, "alluser");

    router.get("/helloworld",  |req: &mut Request|hello_world(req), "helloworld");



      let mut mount = Mount::new();

    {
        let _ = mount
            .mount("/api/", router)
            .mount("/", Static::new(Path::new("web/")))
        ;
    }



    let mut serv = Iron::new(mount).http(format!("localhost:{}", port)).unwrap();
    return serv;
}



pub mod responsehandlers {
    use super::*;
    use manager::*;

    pub fn all_users(backend : &RwLock<Backend>, req: &mut iron::request::Request) -> IronResult<Response> {

        let map = req.get_ref::<Params>().unwrap();

        match map.find(&["query"]) {
            Some(&Value::String(ref json)) => {

                let dat = backend.read().unwrap();

                let param : ParametersAllUsers = serde_json::from_str(json).unwrap();


                let mut v : Vec<rustix_bl::datastore::User> = Vec::new();
                let mut total = 0u32; //TODO: implement correctly in manager


                let hm = &dat.datastore.users;

                for (id, user) in &(*dat).datastore.users {
                    //if user.username.contains(param.count_pars.searchterm) {
                        total += 1;
                        v.push(user.clone());
                    //}
                }


                let result : PaginatedResult<rustix_bl::datastore::User> = PaginatedResult {
                    total_count: total,
                    from: param.pagination.start_inclusive,
                    to: param.pagination.end_exclusive,
                    results: v,
                };


                let json = serde_json::to_string(&result).unwrap();






                Ok(Response::with((iron::status::Ok, json)))
            },
            _ => Ok(Response::with(iron::status::BadRequest)),
        }


    }

    pub fn top_users(backend : &RwLock<Backend>, req: &mut iron::request::Request) -> IronResult<Response> {
        Ok(Response::with((iron::status::Ok, "Hello World")))
    }
}


pub fn execute_cervisia_server(with_config: &ServerConfig,
                               old_backend : Option<Arc<RwLock<Backend>>>,
                               old_server : Option<iron::Listening>) -> (Arc<RwLock<Backend>>, iron::Listening) {

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

    let mut backend = Arc::new(RwLock::new(rustix_bl::build_transient_backend()));


    info!("Building server");

    let backend2 = backend.clone();

    let mut server = build_server(backend2, with_config.server_port);

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


#[derive(Serialize, Deserialize)]
pub struct PaginatedResult<T> {
    total_count : u32,
    from : u32,
    to : u32,
    results : Vec<T>,
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

fn apply_write(backend : &mut Backend, event : rustix_event_shop::BLEvents ) -> Result<SuccessContent, Self::ErrorType>;
    fn apply_write_to_result(backend : &mut Backend, event : rustix_event_shop::BLEvents) -> ServerWriteResult {
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
    use std::thread;
    use std::sync::{Arc, Mutex};
    use std::sync::mpsc::channel;


    const HOST_WITHOUTPORT : &'static str = "http://localhost:";


    lazy_static! {

    static ref PORTCOUNTER: Mutex<u16> = Mutex::new(8081);

}

    fn get_and_increment_port() -> u16 {
        let mut data = PORTCOUNTER.lock().unwrap();
        let old_port : u16 = *data;
        *data = old_port + 1;
        return old_port;
    }

    fn get_server_config() -> ServerConfig {

        return ServerConfig {

        use_send_mail: false,
        email_server: String::new(),
        email_username: String::new(),
        email_password: String::new(),
        top_items_per_user: 4,
        server_port: get_and_increment_port(),
    };}

    fn build_default_server() -> (Arc<RwLock<Backend>>, iron::Listening, ServerConfig) {
        let default_server_conf = get_server_config();
        let (a,b) = execute_cervisia_server(&default_server_conf, None, None);

        return (a, b, default_server_conf);
    }


    #[test]
    fn it_works() {
        assert!(1+1 == 2);
    }

    #[test]
    fn index_html_works() {

        let (backend, server, config) = build_default_server();

        let httpbody = blocking_http_get_call(&format!("{}{}/index.html", HOST_WITHOUTPORT, config.server_port)).unwrap();

        let mut server = server;
        server.close().unwrap();

        assert!(httpbody.contains("Cervisia Frontend"));

    }


    #[test]
    fn hello_world_works() {

        let (backend, server, config) = build_default_server();

        let httpbody = blocking_http_get_call(&format!("{}{}/api/helloworld", HOST_WITHOUTPORT, config.server_port)).unwrap();

        let mut server = server;
        server.close().unwrap();

        assert_eq!(httpbody, "Hello World");

    }

    #[test]
    fn second_hello_world_works() {

        let (backend, server, config) = build_default_server();

        let httpbody = blocking_http_get_call(&format!("{}{}/api/helloworld", HOST_WITHOUTPORT, config.server_port)).unwrap();

        let mut server = server;
        server.close().unwrap();

        assert_eq!(httpbody, "Hello World");

    }

    #[test]
    fn getting_all_users_works() {

        let (backend, server, config) = build_default_server();
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
        let url = format!("{}{}/api/users/all?query={}", HOST_WITHOUTPORT, config.server_port, query);

        let httpbody = blocking_http_get_call(&url).unwrap();

        server.close().unwrap();


        let parsedjson : PaginatedResult<rustix_bl::datastore::User> = serde_json::from_str(&httpbody).unwrap();

        assert_eq!(parsedjson.results.len(), 53);

    }
}