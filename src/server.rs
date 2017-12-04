use iron::prelude::*;
use iron::{BeforeMiddleware, AfterMiddleware, typemap};
use time;
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
use serde;
use chrono::prelude::*;

use iron::prelude::*;
use iron::status;
use router::Router;
use responsehandlers::*;
use params;
use persistent;
use persistent::State;
use iron::typemap::Key;

use params::{Params, Value};

pub type Backend = rustix_bl::rustix_backend::RustixBackend<rustix_bl::persistencer::TransientPersister>;

#[derive(Copy, Clone)]
pub struct SharedBackend;

impl Key for SharedBackend {
    type Value = Backend;
}


pub fn build_server(port: u16, backend: Option<Backend>) -> iron::Listening {
    let mut router = Router::new();


    router.get("/users/all", all_users, "alluser");
    router.post("/users", add_user, "adduser");
    router.post("/purchases", simple_purchase, "addsimplepurchase");

    router.get("/helloworld", hello_world, "helloworld");


    let mut mount = Mount::new();

    {
        let mut chain = Chain::new(router);

        let backend = backend.unwrap_or(rustix_bl::build_transient_backend());
        let state = State::<SharedBackend>::both(backend);

        chain.link(state);

        let _ = mount
            .mount("/api/", chain)
            .mount("/", Static::new(Path::new("web/")))
        ;
    }


    let mut serv = Iron::new(mount).http(format!("localhost:{}", port)).unwrap();
    return serv;
}


pub mod responsehandlers {
    use super::*;
    use manager::*;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct CreateItem {
        itemname: String,
        price_cents: u32,
        category: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct CreateUser { pub username: String }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct DeleteItem { pub item_id: u32 }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct DeleteUser { pub user_id: u32 }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct MakeSimplePurchase {
        pub user_id: u32,
        pub item_id: u32,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct UndoPurchase { pub unique_id: u64 }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct CreateBill {
        pub user_ids: rustix_bl::datastore::UserGroup,
        pub comment: String,
    }

    fn extract_query(req: &mut iron::request::Request) -> Option<String> {
        let map = req.get_ref::<Params>().unwrap();
        return match map.find(&["query"]) {
            Some(&Value::String(ref json)) => {
                return Some(json.to_string());
            }
            _ => None
        };
    }


    fn extract_body(req: &mut iron::request::Request) -> String {
        let mut s = String::new();
        let number_of_bytes = req.body.read_to_string(&mut s);
        return s;
    }


    pub fn simple_purchase(req: &mut iron::request::Request) -> IronResult<Response> {
        let posted_body = extract_body(req);
        println!("posted_body = {:?}", posted_body);
        let parsed_body: MakeSimplePurchase = serde_json::from_str(&posted_body).unwrap();
        let datholder = req.get::<State<SharedBackend>>().unwrap();
        let mut dat = datholder.write().unwrap();
        let query_str = extract_query(req);

        match query_str {
            Some(json_query) => {
                let param: ParametersAll = serde_json::from_str(&json_query).unwrap();

                let result = ServableRustixImpl::check_apply_write(&mut dat, param, rustix_bl::rustix_event_shop::BLEvents::MakeSimplePurchase {
                    user_id: parsed_body.user_id,
                    item_id: parsed_body.item_id,
                    timestamp: current_time_millis(),
                });

                match result {
                    Ok(sux) => return Ok(Response::with((iron::status::Ok, serde_json::to_string(&ServerWriteResult {
                        error_message: None,
                        is_success: true,
                        content: Some(SuccessContent {
                            timestamp_epoch_millis: current_time_millis(),
                            refreshed_data: sux,
                        }),
                    }).unwrap()))),
                    Err(err) => return Ok(Response::with((iron::status::Conflict, serde_json::to_string(&ServerWriteResult {
                        error_message: Some(err.description().to_string()),
                        is_success: false,
                        content: None,
                    }).unwrap()))),
                }
            }
            _ => return Ok(Response::with(iron::status::BadRequest)),
        };
    }

    pub fn add_user(req: &mut iron::request::Request) -> IronResult<Response> {
        let posted_body = extract_body(req);
        println!("posted_body = {:?}", posted_body);
        let parsed_body: CreateUser = serde_json::from_str(&posted_body).unwrap();
        let datholder = req.get::<State<SharedBackend>>().unwrap();
        let mut dat = datholder.write().unwrap();
        let query_str = extract_query(req);

        match query_str {
            Some(json_query) => {
                let param: ParametersAll = serde_json::from_str(&json_query).unwrap();

                let result = ServableRustixImpl::check_apply_write(&mut dat, param, rustix_bl::rustix_event_shop::BLEvents::CreateUser {
                    username: parsed_body.username,
                });

                match result {
                    Ok(sux) => return Ok(Response::with((iron::status::Ok, serde_json::to_string(&ServerWriteResult {
                        error_message: None,
                        is_success: true,
                        content: Some(SuccessContent {
                            timestamp_epoch_millis: current_time_millis(),
                            refreshed_data: sux,
                        }),
                    }).unwrap()))),
                    Err(err) => return Ok(Response::with((iron::status::Conflict, serde_json::to_string(&ServerWriteResult {
                        error_message: Some(err.description().to_string()),
                        is_success: false,
                        content: None,
                    }).unwrap()))),
                }
            }
            _ => return Ok(Response::with(iron::status::BadRequest)),
        };
    }

    pub fn all_users(req: &mut iron::request::Request) -> IronResult<Response> {
        let datholder = req.get::<State<SharedBackend>>().unwrap();
        let dat = datholder.read().unwrap();
        let query_str = extract_query(req);

        match query_str {
            Some(json_query) => {
                let param: ParametersAllUsers = serde_json::from_str(&json_query).unwrap();

                let result = ServableRustixImpl::query_read(&dat, ReadQueryParams::AllUsers(param));

                match result {
                    Ok(sux) => return Ok(Response::with((iron::status::Ok, serde_json::to_string(&sux).unwrap()))),
                    Err(err) => return Ok(Response::with((iron::status::Conflict, serde_json::to_string(&PaginatedResult::<rustix_bl::datastore::User> {
                        total_count: 0,
                        from: 0,
                        to: 0,
                        results: Vec::new(),
                    }).unwrap()))),
                }
            }
            _ => return Ok(Response::with(iron::status::BadRequest)),
        };
    }

    pub fn top_users(backend: &RwLock<Backend>, req: &mut iron::request::Request) -> IronResult<Response> {
        Ok(Response::with((iron::status::Ok, "Hello World")))
    }
}


pub fn execute_cervisia_server(with_config: &ServerConfig,
                               old_server: Option<iron::Listening>, backend: Option<Backend>) -> (iron::Listening) {
    info!("execute_cervisia_server begins for config = {:?}", with_config);

    if old_server.is_some() {
        info!("Closing old server");
        //TODO: does not work, see https://github.com/hyperium/hyper/issues/338
        old_server.unwrap().close().unwrap();
    };


    info!("Building server");

    let mut server = build_server(with_config.server_port, backend);

    println!("Having built server");


    return server;
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ServerWriteResult {
    pub error_message: Option<String>,
    pub is_success: bool,
    pub content: Option<SuccessContent>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SuccessContent {
    pub timestamp_epoch_millis: i64,
    pub refreshed_data: RefreshedData,

}

pub fn get_current_milliseconds() -> i64 {
    return current_time_millis();
}

pub fn current_time_millis() -> i64 {
    let d = Local::now();
    return (d.timestamp() * 1000) + (d.nanosecond() as i64 / 1000000);
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RefreshedData {
    pub DetailInfoForUser: serde_json::Value,
    pub TopUsers: serde_json::Value,
    pub AllUsers: serde_json::Value,
    pub AllItems: serde_json::Value,
    pub PurchaseLogGlobal: serde_json::Value,
    pub BillsCount: serde_json::Value,
    pub Bills: serde_json::Value,
    pub OpenFFAFreebies: serde_json::Value,
    pub TopPersonalDrinks: serde_json::Value,
    pub PurchaseLogPersonal: serde_json::Value,
    pub IncomingFreebies: serde_json::Value,
    pub OutgoingFreebies: serde_json::Value,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaginatedResult<T> {
    pub total_count: u32,
    pub from: u32,
    pub to: u32,
    pub results: Vec<T>,
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
    type ErrorType: std::error::Error;

    fn apply_write(backend: &mut Backend, event: rustix_event_shop::BLEvents) -> Result<SuccessContent, Self::ErrorType>;
    fn apply_write_to_result(backend: &mut Backend, event: rustix_event_shop::BLEvents) -> ServerWriteResult {
        let r = Self::apply_write(backend, event);
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

    let mut s: String = "".to_string();
    let size = res.read_to_string(&mut s);

    println!("Body:\n{}", s);

    println!("\n\nDone.");
    return Ok(s);
}


pub fn blocking_http_post_call<T: serde::ser::Serialize>(url: &str, content: &T) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let mut res = client.post(url)
        .body(serde_json::to_string(content).unwrap())
        .send()?;


    println!("Status: {}", res.status());
    println!("Headers:\n{}", res.headers());

    let mut s: String = "".to_string();
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
    use manager::tests::*;
    use std::thread;
    use std::sync::{Arc, Mutex};
    use std::sync::mpsc::channel;


    const HOST_WITHOUTPORT: &'static str = "http://localhost:";


    lazy_static! {

    static ref PORTCOUNTER: Mutex<u16> = Mutex::new(8081);

}

    fn get_and_increment_port() -> u16 {
        let mut data = PORTCOUNTER.lock().unwrap();
        let old_port: u16 = *data;
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
        };
    }

    fn build_default_server<T>(function_to_fill_backend: T) -> (iron::Listening, ServerConfig) where T: Fn(&mut Backend) -> () {
        let default_server_conf = get_server_config();

        let mut backend = rustix_bl::build_transient_backend();

        function_to_fill_backend(&mut backend);

        let a = execute_cervisia_server(&default_server_conf, None, Some(backend));

        return (a, default_server_conf);
    }


    #[test]
    fn it_works() {
        assert!(1 + 1 == 2);
    }

    #[test]
    fn index_html_works() {
        let (server, config) = build_default_server(fill_not);

        let httpbody = blocking_http_get_call(&format!("{}{}/index.html", HOST_WITHOUTPORT, config.server_port)).unwrap();

        let mut server = server;
        server.close().unwrap();

        assert!(httpbody.contains("Cervisia Frontend"));
    }


    #[test]
    fn hello_world_works() {
        let (server, config) = build_default_server(fill_not);

        let httpbody = blocking_http_get_call(&format!("{}{}/api/helloworld", HOST_WITHOUTPORT, config.server_port)).unwrap();

        let mut server = server;
        server.close().unwrap();

        assert_eq!(httpbody, "Hello World");
    }

    #[test]
    fn second_hello_world_works() {
        let (server, config) = build_default_server(fill_not);

        let httpbody = blocking_http_get_call(&format!("{}{}/api/helloworld", HOST_WITHOUTPORT, config.server_port)).unwrap();

        let mut server = server;
        server.close().unwrap();

        assert_eq!(httpbody, "Hello World");
    }

    #[test]
    fn getting_all_users_works() {
        let (server, config) = build_default_server(fill_backend_with_medium_test_data);
        let mut server = server;

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


        let parsedjson: PaginatedResult<rustix_bl::datastore::User> = serde_json::from_str(&httpbody).unwrap();

        assert_eq!(parsedjson.results.len(), 53);
    }


    #[test]
    fn adding_a_user_works() {
        let (server, config) = build_default_server(fill_backend_with_medium_test_data);
        let mut server = server;

        let params_for_user = ParametersAllUsers {
            count_pars: ParametersAllUsersCount {
                searchterm: "".to_string(),
            },
            pagination: ParametersPagination {
                start_inclusive: 0,
                end_exclusive: 1_000_000,
            },
        };

        {
            let query = serde_json::to_string(&params_for_user).unwrap();
            let url = format!("{}{}/api/users/all?query={}", HOST_WITHOUTPORT, config.server_port, query);

            let httpbody = blocking_http_get_call(&url).unwrap();

            server.close().unwrap();


            let parsedjson: PaginatedResult<rustix_bl::datastore::User> = serde_json::from_str(&httpbody).unwrap();


            assert_eq!(parsedjson.results.len(), 53);
        }

        let state = ParametersAll {
            top_users: ParametersTopUsers { n: 0 },
            all_users: ParametersAllUsers { count_pars: ParametersAllUsersCount { searchterm: String::new() }, pagination: ParametersPagination { start_inclusive: 0, end_exclusive: 1_000_000 } },
            all_items: ParametersAllItems { count_pars: ParametersAllItemsCount { searchterm: String::new() }, pagination: ParametersPagination { start_inclusive: 0, end_exclusive: 0 } },
            global_log: ParametersPurchaseLogGlobal { count_pars: ParametersPurchaseLogGlobalCount { millis_start: 0, millis_end: 0 }, pagination: ParametersPagination { start_inclusive: 0, end_exclusive: 0 } },
            bills: ParametersBills { count_pars: ParametersBillsCount {}, pagination: ParametersPagination { start_inclusive: 0, end_exclusive: 0 } },
            open_ffa_freebies: ParametersOpenFFAFreebies {},
            top_personal_drinks: ParametersTopPersonalDrinks { user_id: 0, n: 0 },
            personal_log: ParametersPurchaseLogPersonal {
                count_pars: ParametersPurchaseLogPersonalCount {
                    user_id: 0,
                    millis_start: 0,
                    millis_end: 0,
                },
                pagination: ParametersPagination { start_inclusive: 0, end_exclusive: 0 },
            },
            incoming_freebies: ParametersIncomingFreebies {},
            outgoing_freebies: ParametersOutgoingFreebies {},
            personal_detail_infos: ParametersDetailInfoForUser { user_id: 0 },
        };

        let postjson = CreateUser {
            username: "my new username".to_string(),
        };

        let query = serde_json::to_string(&state).unwrap();
        let url = format!("{}{}/api/users?query={}", HOST_WITHOUTPORT, config.server_port, query);

        let httpbody = blocking_http_post_call(&url, &postjson).unwrap();

        server.close().unwrap();


        let parsedjson: ServerWriteResult = serde_json::from_str(&httpbody).unwrap();

        assert_eq!(parsedjson.is_success, true);
        assert_eq!(parsedjson.error_message, None);
        assert!(parsedjson.content.is_some());
        let unpacked = parsedjson.content.unwrap();
        //println!("untracked = {:?}", unpacked);
        assert!(unpacked.refreshed_data.AllUsers.as_object().unwrap().get("results").unwrap().as_array().is_some());
        assert_eq!(unpacked.refreshed_data.AllUsers.as_object().unwrap().get("results").unwrap().as_array().unwrap().len(), 54);


        {
            let query = serde_json::to_string(&params_for_user).unwrap();
            let url = format!("{}{}/api/users/all?query={}", HOST_WITHOUTPORT, config.server_port, query);

            let httpbody = blocking_http_get_call(&url).unwrap();

            server.close().unwrap();


            let parsedjson: PaginatedResult<rustix_bl::datastore::User> = serde_json::from_str(&httpbody).unwrap();

            assert_eq!(parsedjson.results.len(), 54);
        }
    }
    #[test]
    fn making_a_simple_purchase_works() {
        let (server, config) = build_default_server(fill_backend_with_medium_test_data);
        let mut server = server;

        let params_for_user = ParametersAllUsers {
            count_pars: ParametersAllUsersCount {
                searchterm: "".to_string(),
            },
            pagination: ParametersPagination {
                start_inclusive: 0,
                end_exclusive: 1_000_000,
            },
        };

        let state = ParametersAll {
            top_users: ParametersTopUsers { n: 0 },
            all_users: ParametersAllUsers { count_pars: ParametersAllUsersCount { searchterm: String::new() }, pagination: ParametersPagination { start_inclusive: 0, end_exclusive: 1_000_000 } },
            all_items: ParametersAllItems { count_pars: ParametersAllItemsCount { searchterm: String::new() }, pagination: ParametersPagination { start_inclusive: 0, end_exclusive: 0 } },
            global_log: ParametersPurchaseLogGlobal { count_pars: ParametersPurchaseLogGlobalCount { millis_start: 0, millis_end: 0 }, pagination: ParametersPagination { start_inclusive: 0, end_exclusive: 0 } },
            bills: ParametersBills { count_pars: ParametersBillsCount {}, pagination: ParametersPagination { start_inclusive: 0, end_exclusive: 0 } },
            open_ffa_freebies: ParametersOpenFFAFreebies {},
            top_personal_drinks: ParametersTopPersonalDrinks { user_id: 0, n: 0 },
            personal_log: ParametersPurchaseLogPersonal {
                count_pars: ParametersPurchaseLogPersonalCount {
                    user_id: 0,
                    millis_start: 0,
                    millis_end: 0,
                },
                pagination: ParametersPagination { start_inclusive: 0, end_exclusive: 0 },
            },
            incoming_freebies: ParametersIncomingFreebies {},
            outgoing_freebies: ParametersOutgoingFreebies {},
            personal_detail_infos: ParametersDetailInfoForUser { user_id: 0 },
        };

        let postjson = MakeSimplePurchase {
            user_id: 1,
            item_id: 1,
        };

        let query = serde_json::to_string(&state).unwrap();
        let url = format!("{}{}/api/purchases?query={}", HOST_WITHOUTPORT, config.server_port, query);

        let httpbody = blocking_http_post_call(&url, &postjson).unwrap();

        server.close().unwrap();


        let parsedjson: ServerWriteResult = serde_json::from_str(&httpbody).unwrap();

        assert_eq!(parsedjson.is_success, true);
        assert_eq!(parsedjson.error_message, None);
        assert!(parsedjson.content.is_some());
        let unpacked = parsedjson.content.unwrap();

        assert!(!unpacked.refreshed_data.PurchaseLogPersonal.is_null());
        assert!(!unpacked.refreshed_data.PurchaseLogGlobal.is_null());
        assert!(!unpacked.refreshed_data.TopPersonalDrinks.is_null());
        assert!(!unpacked.refreshed_data.TopUsers.is_null());
        assert!(unpacked.refreshed_data.AllUsers.is_null());
        assert!(!unpacked.refreshed_data.DetailInfoForUser.is_null());
        assert!(unpacked.refreshed_data.OutgoingFreebies.is_null());
        assert!(unpacked.refreshed_data.OpenFFAFreebies.is_null());


    }
}