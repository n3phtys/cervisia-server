use iron::prelude::*;
use iron::{BeforeMiddleware, AfterMiddleware, typemap};
use time;
use time::precise_time_ns;
use std::path::Path;
use chrono::prelude::*;
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
use billformatter;
use std;
use rustix_bl::rustix_event_shop;
use std::sync::{Arc, RwLock};
use std::thread;
use manager::ParametersAll;
use reqwest;
use configuration;
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
use rustix_bl::rustix_backend::WriteBackend;
use manager::fill_backend_with_medium_test_data;
use manager::fill_backend_with_large_test_data;
use rustix_bl::datastore::DatastoreQueries;
use typescriptify::TypeScriptifyTrait;
use manager;
use mail;
use manager::*;

use params::{Params, Value};

pub type Backend = rustix_bl::rustix_backend::RustixBackend;

#[derive(Copy, Clone)]
pub struct SharedBackend;

impl Key for SharedBackend {
    type Value = Backend;
}

fn typescript_definitions() -> Vec<String> {
    return vec![
        ParametersAll::type_script_ify(),
        ParametersPagination::type_script_ify(),
        ParametersTopUsers::type_script_ify(),
        ParametersAllUsersCount::type_script_ify(),
        ParametersAllUsers::type_script_ify(),
        ParametersAllItemsCount::type_script_ify(),
        ParametersAllItems::type_script_ify(),
        ParametersPurchaseLogGlobalCount::type_script_ify(),
        ParametersPurchaseLogGlobal::type_script_ify(),
        ParametersBillsCount::type_script_ify(),
        ParametersBills::type_script_ify(),
        ParametersBillDetails::type_script_ify(),
        ParametersOpenFFAFreebies::type_script_ify(),
        ParametersTopPersonalDrinks::type_script_ify(),
        ParametersPurchaseLogPersonalCount::type_script_ify(),
        ParametersPurchaseLogPersonal::type_script_ify(),
        ParametersIncomingFreebiesCount::type_script_ify(),
        ParametersIncomingFreebies::type_script_ify(),
        ParametersOutgoingFreebiesCount::type_script_ify(),
        ParametersOutgoingFreebies::type_script_ify(),
        ParametersDetailInfoForUser::type_script_ify(),
        EnrichedFFA::type_script_ify(),
        UserDetailInfo::type_script_ify(),
        DetailedBill::type_script_ify(),
        manager::Purchase::type_script_ify(),
        CreateBill::type_script_ify(),
        EditBill::type_script_ify(),
        FinalizeBill::type_script_ify(),
        ExportBill::type_script_ify(),
        DeleteUnfinishedBill::type_script_ify(),
        DeleteUnfinishedBill::type_script_ify(),
        rustix_bl::datastore::User::type_script_ify(),
        rustix_bl::datastore::UserGroup::type_script_ify(),
        rustix_bl::datastore::Item::type_script_ify(),
        rustix_bl::datastore::BillState::type_script_ify(),
        rustix_bl::datastore::BillUserInstance::type_script_ify(),
        rustix_bl::datastore::BillUserDayInstance::type_script_ify(),
        rustix_bl::datastore::ExportableBillData::type_script_ify(),
        rustix_bl::datastore::PricedSpecial::type_script_ify(),
        rustix_bl::datastore::PaidFor::type_script_ify(),
        rustix_bl::datastore::Bill::type_script_ify(),
        rustix_bl::datastore::Freeby::type_script_ify(),
        ServerWriteResult::type_script_ify(),
        SuccessContent::type_script_ify(),
        RefreshedData::type_script_ify(),
        responsehandlers::MakeSimplePurchase::type_script_ify(),
        responsehandlers::MakeCartPurchase::type_script_ify(),
        responsehandlers::MakeFFAPurchase::type_script_ify(),
        responsehandlers::CreateFreeForAll::type_script_ify(),
        responsehandlers::CreateBudgetGiveout::type_script_ify(),
        responsehandlers::CreateCountGiveout::type_script_ify(),
        responsehandlers::SetPriceForSpecial::type_script_ify(),
        responsehandlers::KeyValue::type_script_ify(),
    ];
}

fn typescript_definition_string() -> String {
    let mut s = "".to_string();
    for x in typescript_definitions() {
        s = s + x.as_ref() + "\n\n";
    }
    return s;
}


pub fn build_server(config: &ServerConfig, backend: Option<Backend>) -> iron::Listening {
    let mut router = Router::new();


    //let endpoints = typescript_definition_string();
    router.get("/endpoints", |req: &mut iron::request::Request| Ok(Response::with((iron::status::Ok, typescript_definition_string()))), "endpoints");


    router.get("/users/all", all_users, "allusers");
    router.get("/users/top", top_users, "topusers");
    router.get("/users/detail", user_detail_info, "userdetails");
    router.get("/items/top", top_items, "topitems");
    router.get("/items/all", all_items, "allitems");
    router.get("/purchases/global", global_log, "globallog");
    router.get("/purchases/personal", personal_log, "personallog");
    router.get("/bills", get_bills, "getbills");
    router.get("/bills/detail", get_detailed_bill, "getdetailedbill");
    router.get("/giveout/ffa", get_ffa_giveouts, "ffagiveouts");
    router.get("/giveout/incoming", get_incoming_giveouts, "incominggiveouts");
    router.get("/giveout/outgoing", get_outgoing_giveouts, "outgoinggiveouts");
    router.post("/users", add_user, "adduser");
    router.post("/items", add_item, "additem");
    router.post("/users/update", update_user, "updateuser");
    router.post("/items/update", update_item, "updateitem");
    router.post("/users/delete", delete_user, "deleteuser");
    router.post("/items/delete", delete_item, "deleteitem");
    router.post("/purchases", simple_purchase, "addsimplepurchase");
    router.post("/purchases/cart", cart_purchase, "addcartpurchase");
    router.post("/purchases/ffa", ffa_purchase, "addffapurchase");
    router.post("/purchases/undo/user", undo_purchase_by_user, "undopurchaseuser");
    router.post("/purchases/undo/admin", undo_purchase_by_admin, "undopurchaseadmin");

    router.post("/bill/create", create_bill, "createbill");
    router.post("/bill/update", update_bill, "updatebill");
    router.post("/bill/delete", delete_bill, "deletebill");
    router.post("/bill/finalize", finalize_bill, "finalizebill");

    {
        let config = config.clone();
        router.post("/bill/export", move |req: &mut iron::request::Request| {
            let conf = config.clone();
            export_bill(req, &conf )
        }, "exportbill");

    }

    router.post("/purchases/special/setprice", set_special_price, "setspecialprice");

    router.post("/giveout/budget", create_budget_freeby, "createbudgetfreeby");
    router.post("/giveout/count", create_count_freeby, "createcountfreeby");
    router.post("/giveout/ffa", create_ffa_freeby, "createffafreeby");

    router.get("/helloworld", hello_world, "helloworld");


    let mut mount = Mount::new();

    {
        let mut chain = Chain::new(router);

        let fill = backend.is_none();


        let mut backend = backend.unwrap_or( if config.use_persistence {
            let mut b = rustix_bl::build_persistent_backend(std::path::Path::new(&config.persistence_file_path));
            let c = b.reload().unwrap();
            if c == 0 && fill {
                fill_backend_with_large_test_data(&mut b); //TODO: replace for production
            }
            b
        } else {
            let mut b = rustix_bl::build_transient_backend();

            if fill {
                fill_backend_with_large_test_data(&mut b); //TODO: replace for production
            }
            b
        });

        let state = State::<SharedBackend>::both(backend);

        chain.link(state);

        let _ = mount
            .mount("/api/", chain)
            .mount("/", Static::new(Path::new(&config.web_path)))
        ;
    }

    let url = format!("{}:{}", config.host, config.server_port);
    println!("Starting server under host and port = {}", &url);
    let mut serv = Iron::new(mount).http(url).unwrap();
    return serv;
}


pub mod responsehandlers {
    use super::*;
    use manager::*;
    use billformatter::BillFormatting;

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct CreateItem {
        pub name: String,
        pub price_cents: u32,
        pub category: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct CreateUser { pub username: String }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct UpdateItem {
        pub name: String,
        pub item_id: u32,
        pub category: Option<String>,
        pub price_cents: u32,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct UpdateUser {
        pub username: String,
        pub external_user_id: Option<String>,
        pub user_id: u32,
        pub is_billed: bool,
        pub highlight_in_ui: bool,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, TypeScriptify)]
    pub struct DeleteItem { pub item_id: u32 }

    #[derive(Serialize, Deserialize, Debug, Clone, TypeScriptify)]
    pub struct DeleteUser { pub user_id: u32 }

    #[derive(Serialize, Deserialize, Debug, Clone, TypeScriptify)]
    pub struct MakeSimplePurchase {
        pub user_id: u32,
        pub item_id: u32,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, TypeScriptify)]
    pub struct KeyValue {
        pub key: u32,
        pub value: u32,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, TypeScriptify)]
    pub struct MakeCartPurchase {
        pub user_id: u32,
        pub items: Vec<KeyValue>,
        pub specials: Vec<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, TypeScriptify)]
    pub struct UndoPurchase { pub unique_id: u64 }

    #[derive(Serialize, Deserialize, Debug, Clone, TypeScriptify)]
    pub struct CreateBill {
        pub timestamp_from: i64,
        pub timestamp_to: i64,
        pub comment: String,
    }


    #[derive(Serialize, Deserialize, Debug, Clone, TypeScriptify)]
    pub struct EditBill {
        pub timestamp_from: i64,
        pub timestamp_to: i64,
        pub comment: String,
        pub exclude_user_ids: HashSet<u32>,
    }


    #[derive(Serialize, Deserialize, Debug, Clone, TypeScriptify)]
    pub struct FinalizeBill {
        pub timestamp_from: i64,
        pub timestamp_to: i64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, TypeScriptify)]
    pub struct ExportBill {
        pub timestamp_from: i64,
        pub timestamp_to: i64,
        pub limit_to_user: Option<u32>,
        pub email_address: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, TypeScriptify)]
    pub struct DeleteUnfinishedBill {
        pub timestamp_from: i64,
        pub timestamp_to: i64,
    }


    #[derive(Serialize, Deserialize, Debug, Clone, TypeScriptify)]
    pub struct MakeFFAPurchase {
        pub ffa_id: u64,
        pub item_id: u32,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, TypeScriptify)]
    pub struct CreateFreeForAll {
        pub allowed_categories: Vec<String>,
        pub allowed_drinks: Vec<u32>,
        pub allowed_number_total: u16,
        pub text_message: String,
        pub donor: u32,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, TypeScriptify)]
    pub struct CreateBudgetGiveout {
        pub cents_worth_total: u64,
        pub text_message: String,
        pub donor: u32,
        pub recipient: u32,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, TypeScriptify)]
    pub struct CreateCountGiveout {
        pub allowed_categories: Vec<String>,
        pub allowed_drinks: Vec<u32>,
        pub allowed_number_total: u16,
        pub text_message: String,
        pub donor: u32,
        pub recipient: u32,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, TypeScriptify)]
    pub struct SetPriceForSpecial {
        pub unique_id: u64,
        pub price: u32,
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


    pub fn undo_purchase_by_user(req: &mut iron::request::Request) -> IronResult<Response> {
        let posted_body = extract_body(req);
        println!("posted_body = {:?}", posted_body);
        let parsed_body: UndoPurchase = serde_json::from_str(&posted_body).unwrap();
        let datholder = req.get::<State<SharedBackend>>().unwrap();
        let mut dat = datholder.write().unwrap();
        let query_str = extract_query(req);

        match query_str {
            Some(json_query) => {
                let param: ParametersAll = serde_json::from_str(&json_query).unwrap();

                let cur = current_time_millis();

                if match dat.datastore.get_purchase_timestamp(parsed_body.unique_id) {
                    Some(ref t) => {
                        cur < t + (30i64 * 1000i64)
                    },
                    None => false,
                }  {
                    return Ok(Response::with((iron::status::Conflict, serde_json::to_string(&ServerWriteResult {
                        error_message: Some("A user may only undo a purchase before 30s have passed".to_string()),
                        is_success: false,
                        content: None,
                    }).unwrap())));
                } else {
                    let result = ServableRustixImpl::check_apply_write(&mut dat, param, rustix_bl::rustix_event_shop::BLEvents::UndoPurchase {
                        unique_id: parsed_body.unique_id,
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
            }
            _ => return Ok(Response::with(iron::status::BadRequest)),
        };
    }

    pub fn undo_purchase_by_admin(req: &mut iron::request::Request) -> IronResult<Response> {
        let posted_body = extract_body(req);
        println!("posted_body = {:?}", posted_body);
        let parsed_body: UndoPurchase = serde_json::from_str(&posted_body).unwrap();
        let datholder = req.get::<State<SharedBackend>>().unwrap();
        let mut dat = datholder.write().unwrap();
        let query_str = extract_query(req);

        match query_str {
            Some(json_query) => {
                let param: ParametersAll = serde_json::from_str(&json_query).unwrap();

                let cur = current_time_millis();

                if dat.datastore.get_purchase_timestamp(parsed_body.unique_id).is_some() {
                    return Ok(Response::with((iron::status::Conflict, serde_json::to_string(&ServerWriteResult {
                        error_message: Some("Cannot find purchase to delete (the purchase may have already been finalized into a bill, undoing such a purchase is not possible)".to_string()),
                        is_success: false,
                        content: None,
                    }).unwrap())));
                } else {
                    let result = ServableRustixImpl::check_apply_write(&mut dat, param, rustix_bl::rustix_event_shop::BLEvents::UndoPurchase {
                        unique_id: parsed_body.unique_id,
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
            }
            _ => return Ok(Response::with(iron::status::BadRequest)),
        };
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


    pub fn cart_purchase(req: &mut iron::request::Request) -> IronResult<Response> {
        let posted_body = extract_body(req);
        println!("posted_body = {:?}", posted_body);
        let parsed_body: MakeCartPurchase = serde_json::from_str(&posted_body).unwrap();
        let datholder = req.get::<State<SharedBackend>>().unwrap();
        let mut dat = datholder.write().unwrap();
        let query_str = extract_query(req);

        match query_str {
            Some(json_query) => {
                let param: ParametersAll = serde_json::from_str(&json_query).unwrap();

                let mut item_ids: Vec<u32> = Vec::new();
                for kv in parsed_body.items {
                    for i in 1..kv.value {
                        item_ids.push(kv.key);
                    }
                }

                let event = rustix_bl::rustix_event_shop::BLEvents::MakeShoppingCartPurchase {
                    user_id: parsed_body.user_id,
                    specials: parsed_body.specials,
                    item_ids: item_ids,
                    timestamp: current_time_millis(),
                };

                let result = ServableRustixImpl::check_apply_write(&mut dat, param, event);

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



    pub fn ffa_purchase(req: &mut iron::request::Request) -> IronResult<Response> {
        let posted_body = extract_body(req);
        println!("posted_body = {:?}", posted_body);
        let parsed_body: MakeFFAPurchase = serde_json::from_str(&posted_body).unwrap();
        let datholder = req.get::<State<SharedBackend>>().unwrap();
        let mut dat = datholder.write().unwrap();
        let query_str = extract_query(req);

        match query_str {
            Some(json_query) => {
                let param: ParametersAll = serde_json::from_str(&json_query).unwrap();

                let event = rustix_bl::rustix_event_shop::BLEvents::MakeFreeForAllPurchase {
                    ffa_id: parsed_body.ffa_id,
                    item_id: parsed_body.item_id,
                    timestamp: current_time_millis(),
                };

                let result = ServableRustixImpl::check_apply_write(&mut dat, param, event);

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



    pub fn create_budget_freeby(req: &mut iron::request::Request) -> IronResult<Response> {
        let posted_body = extract_body(req);
        println!("posted_body = {:?}", posted_body);
        let parsed_body: CreateBudgetGiveout = serde_json::from_str(&posted_body).unwrap();
        let datholder = req.get::<State<SharedBackend>>().unwrap();
        let mut dat = datholder.write().unwrap();
        let query_str = extract_query(req);

        match query_str {
            Some(json_query) => {
                let param: ParametersAll = serde_json::from_str(&json_query).unwrap();

                let event = rustix_bl::rustix_event_shop::BLEvents::CreateFreeBudget {
                    cents_worth_total: parsed_body.cents_worth_total,
                    text_message: parsed_body.text_message,
                    created_timestamp: current_time_millis(),
                    donor: parsed_body.donor,
                    recipient: parsed_body.recipient,
                };

                let result = ServableRustixImpl::check_apply_write(&mut dat, param, event);

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



    pub fn create_count_freeby(req: &mut iron::request::Request) -> IronResult<Response> {
        let posted_body = extract_body(req);
        println!("posted_body = {:?}", posted_body);
        let parsed_body: CreateCountGiveout = serde_json::from_str(&posted_body).unwrap();
        let datholder = req.get::<State<SharedBackend>>().unwrap();
        let mut dat = datholder.write().unwrap();
        let query_str = extract_query(req);

        match query_str {
            Some(json_query) => {
                let param: ParametersAll = serde_json::from_str(&json_query).unwrap();

                let event = rustix_bl::rustix_event_shop::BLEvents::CreateFreeCount {
                    allowed_categories: parsed_body.allowed_categories,
                    allowed_drinks: parsed_body.allowed_drinks,
                    allowed_number_total: parsed_body.allowed_number_total,
                    text_message: parsed_body.text_message,
                    created_timestamp: current_time_millis(),
                    donor: parsed_body.donor,
                    recipient: parsed_body.recipient,
                };

                let result = ServableRustixImpl::check_apply_write(&mut dat, param, event);

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



    pub fn create_ffa_freeby(req: &mut iron::request::Request) -> IronResult<Response> {
        let posted_body = extract_body(req);
        println!("posted_body = {:?}", posted_body);
        let parsed_body: CreateFreeForAll = serde_json::from_str(&posted_body).unwrap();
        let datholder = req.get::<State<SharedBackend>>().unwrap();
        let mut dat = datholder.write().unwrap();
        let query_str = extract_query(req);

        match query_str {
            Some(json_query) => {
                let param: ParametersAll = serde_json::from_str(&json_query).unwrap();


                let event = rustix_bl::rustix_event_shop::BLEvents::CreateFreeForAll {
                    allowed_categories: parsed_body.allowed_categories,
                    allowed_drinks: parsed_body.allowed_drinks,
                    allowed_number_total: parsed_body.allowed_number_total,
                    text_message: parsed_body.text_message,
                    created_timestamp: current_time_millis(),
                    donor: parsed_body.donor,
                };

                let result = ServableRustixImpl::check_apply_write(&mut dat, param, event);

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


    pub fn delete_user(req: &mut iron::request::Request) -> IronResult<Response> {
        let posted_body = extract_body(req);
        println!("posted_body = {:?}", posted_body);
        let parsed_body: DeleteUser = serde_json::from_str(&posted_body).unwrap();
        let datholder = req.get::<State<SharedBackend>>().unwrap();
        let mut dat = datholder.write().unwrap();
        let query_str = extract_query(req);

        match query_str {
            Some(json_query) => {
                let param: ParametersAll = serde_json::from_str(&json_query).unwrap();

                let result = ServableRustixImpl::check_apply_write(&mut dat, param, rustix_bl::rustix_event_shop::BLEvents::DeleteUser {
                    user_id: parsed_body.user_id,
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


    pub fn delete_item(req: &mut iron::request::Request) -> IronResult<Response> {
        let posted_body = extract_body(req);
        println!("posted_body = {:?}", posted_body);
        let parsed_body: DeleteItem = serde_json::from_str(&posted_body).unwrap();
        let datholder = req.get::<State<SharedBackend>>().unwrap();
        let mut dat = datholder.write().unwrap();
        let query_str = extract_query(req);

        match query_str {
            Some(json_query) => {
                let param: ParametersAll = serde_json::from_str(&json_query).unwrap();

                let result = ServableRustixImpl::check_apply_write(&mut dat, param, rustix_bl::rustix_event_shop::BLEvents::DeleteItem {
                    item_id: parsed_body.item_id,
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

    pub fn update_user(req: &mut iron::request::Request) -> IronResult<Response> {
        let posted_body = extract_body(req);
        println!("posted_body = {:?}", posted_body);
        let parsed_body: UpdateUser = serde_json::from_str(&posted_body).unwrap();
        let datholder = req.get::<State<SharedBackend>>().unwrap();
        let mut dat = datholder.write().unwrap();
        let query_str = extract_query(req);

        match query_str {
            Some(json_query) => {
                let param: ParametersAll = serde_json::from_str(&json_query).unwrap();

                let result = ServableRustixImpl::check_apply_write(&mut dat, param, rustix_bl::rustix_event_shop::BLEvents::UpdateUser {
                    user_id: parsed_body.user_id,
                    username: parsed_body.username,
                    is_billed: parsed_body.is_billed,
                    is_highlighted: parsed_body.highlight_in_ui,
                    external_user_id: parsed_body.external_user_id,
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

    pub fn add_item(req: &mut iron::request::Request) -> IronResult<Response> {
        let posted_body = extract_body(req);
        println!("posted_body = {:?}", posted_body);
        let parsed_body: CreateItem = serde_json::from_str(&posted_body).unwrap();
        let datholder = req.get::<State<SharedBackend>>().unwrap();
        let mut dat = datholder.write().unwrap();
        let query_str = extract_query(req);

        match query_str {
            Some(json_query) => {
                let param: ParametersAll = serde_json::from_str(&json_query).unwrap();

                let result = ServableRustixImpl::check_apply_write(&mut dat, param, rustix_bl::rustix_event_shop::BLEvents::CreateItem {
                    itemname: parsed_body.name,
                    price_cents: parsed_body.price_cents,
                    category: parsed_body.category,
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

    pub fn create_bill(req: &mut iron::request::Request) -> IronResult<Response> {
        let posted_body = extract_body(req);
        println!("posted_body = {:?}", posted_body);
        let parsed_body: CreateBill = serde_json::from_str(&posted_body).unwrap();
        let datholder = req.get::<State<SharedBackend>>().unwrap();
        let mut dat = datholder.write().unwrap();
        let query_str = extract_query(req);

        match query_str {
            Some(json_query) => {
                let param: ParametersAll = serde_json::from_str(&json_query).unwrap();

                let result = ServableRustixImpl::check_apply_write(&mut dat, param, rustix_bl::rustix_event_shop::BLEvents::CreateBill {
                    timestamp_from: parsed_body.timestamp_from,
                    timestamp_to: parsed_body.timestamp_to,
                    user_ids: rustix_bl::datastore::UserGroup::AllUsers{},
                    comment: parsed_body.comment,
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

    pub fn update_bill(req: &mut iron::request::Request) -> IronResult<Response> {
        let posted_body = extract_body(req);
        println!("posted_body = {:?}", posted_body);
        let parsed_body: EditBill = serde_json::from_str(&posted_body).unwrap();
        let datholder = req.get::<State<SharedBackend>>().unwrap();
        let mut dat = datholder.write().unwrap();
        let query_str = extract_query(req);

        match query_str {
            Some(json_query) => {
                let param: ParametersAll = serde_json::from_str(&json_query).unwrap();

                let result = ServableRustixImpl::check_apply_write(&mut dat, param, rustix_bl::rustix_event_shop::BLEvents::UpdateBill {
                    timestamp_from: parsed_body.timestamp_from,
                    timestamp_to: parsed_body.timestamp_to,
                    comment: parsed_body.comment,
                    users: rustix_bl::datastore::UserGroup::AllUsers{},
                    users_that_will_not_be_billed: parsed_body.exclude_user_ids,
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

    pub fn delete_bill(req: &mut iron::request::Request) -> IronResult<Response> {
        let posted_body = extract_body(req);
        println!("posted_body = {:?}", posted_body);
        let parsed_body: DeleteUnfinishedBill = serde_json::from_str(&posted_body).unwrap();
        let datholder = req.get::<State<SharedBackend>>().unwrap();
        let mut dat = datholder.write().unwrap();
        let query_str = extract_query(req);

        match query_str {
            Some(json_query) => {
                let param: ParametersAll = serde_json::from_str(&json_query).unwrap();

                let result = ServableRustixImpl::check_apply_write(&mut dat, param, rustix_bl::rustix_event_shop::BLEvents::DeleteUnfinishedBill {
                    timestamp_from: parsed_body.timestamp_from,
                    timestamp_to: parsed_body.timestamp_to,
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

    pub fn finalize_bill(req: &mut iron::request::Request) -> IronResult<Response> {
        let posted_body = extract_body(req);
        println!("posted_body = {:?}", posted_body);
        let parsed_body: FinalizeBill = serde_json::from_str(&posted_body).unwrap();
        let datholder = req.get::<State<SharedBackend>>().unwrap();
        let mut dat = datholder.write().unwrap();
        let query_str = extract_query(req);

        match query_str {
            Some(json_query) => {
                let param: ParametersAll = serde_json::from_str(&json_query).unwrap();

                let result = ServableRustixImpl::check_apply_write(&mut dat, param, rustix_bl::rustix_event_shop::BLEvents::FinalizeBill {
                    timestamp_from: parsed_body.timestamp_from,
                    timestamp_to: parsed_body.timestamp_to,
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

    pub fn export_bill(req: &mut iron::request::Request, conf: &configuration::ServerConfig) -> IronResult<Response> {
        let posted_body = extract_body(req);
        println!("posted_body = {:?}", posted_body);
        let parsed_body: ExportBill = serde_json::from_str(&posted_body).unwrap();
        let datholder = req.get::<State<SharedBackend>>().unwrap();
        let mut dat = datholder.write().unwrap();
        let query_str = extract_query(req);

        match query_str {
            Some(json_query) => {
                let param: ParametersAll = serde_json::from_str(&json_query).unwrap();

                let result = ServableRustixImpl::check_apply_write(&mut dat, param, rustix_bl::rustix_event_shop::BLEvents::ExportBill {
                    timestamp_from: parsed_body.timestamp_from,
                    timestamp_to: parsed_body.timestamp_to,
                });


                match result {
                    Ok(sux) => {

                        let bill: rustix_bl::datastore::Bill = dat.datastore.get_bill(parsed_body.timestamp_from, parsed_body.timestamp_to).unwrap().clone();

                        match parsed_body.limit_to_user {
                            Some(user_id) => {
                                let subject = format!("Your Cervisia bill export on {}", Utc::now().format("%d.%m.%Y"));
                                let body_cells = bill.format_as_personalized_documentation(user_id);
                                //TODO: replace delimiter by making it configurable

                                let mut lines: Vec<String> = Vec::new();

                                for line_vec in body_cells {
                                    lines.push(line_vec.join(";"));
                                }
                                let body: String = lines.join("\n");

                                let attachments: HashMap<String, String> = {
                                    let mut hm = HashMap::new();
                                    hm.insert("exported_bill.csv".to_string(), body);
                                    hm
                                };

                                mail::send_mail(&parsed_body.email_address, &subject, "Your bill is attached to this mail as a CSV file", &attachments, conf).unwrap();
                            },
                            None => {
                                let subject = format!("Cervisia bill export on {}", Utc::now().format("%d.%m.%Y"));
                                // construct csv to attach to mail
                                let body_a_cells = bill.format_as_sewobe_csv();
                                // construct total list for all users
                                let body_b_cells = bill.format_as_documentation();

                                // send both to receiver
                                let mut lines_a: Vec<String> = Vec::new();
                                let mut lines_b: Vec<String> = Vec::new();

                                for line_vec in body_a_cells {
                                    lines_a.push(line_vec.join(";"));
                                }
                                let body_a: String = lines_a.join("\n");

                                for line_vec in body_b_cells {
                                    lines_b.push(line_vec.join(";"));
                                }
                                let body_b: String = lines_b.join("\n");

                                let attachments: HashMap<String, String> = {
                                    let mut hm = HashMap::new();
                                    hm.insert("internal_oversight.csv".to_string(), body_b);
                                    hm.insert("sewobe_import.csv".to_string(), body_a);
                                    hm
                                };

                                mail::send_mail(&parsed_body.email_address, &subject, "The bill is attached as two CSV files. One is to import into SEWOBE, the other is for internal tracking and contains additional information.", &attachments, conf).unwrap();
                            },
                        }



                        return Ok(Response::with((iron::status::Ok, serde_json::to_string(&ServerWriteResult {
                            error_message: None,
                            is_success: true,
                            content: Some(SuccessContent {
                                timestamp_epoch_millis: current_time_millis(),
                                refreshed_data: sux,
                            }),
                        }).unwrap())))
                    },
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

    pub fn set_special_price(req: &mut iron::request::Request) -> IronResult<Response> {
        let posted_body = extract_body(req);
        println!("posted_body = {:?}", posted_body);
        let parsed_body: SetPriceForSpecial = serde_json::from_str(&posted_body).unwrap();
        let datholder = req.get::<State<SharedBackend>>().unwrap();
        let mut dat = datholder.write().unwrap();
        let query_str = extract_query(req);

        match query_str {
            Some(json_query) => {
                let param: ParametersAll = serde_json::from_str(&json_query).unwrap();

                let result = ServableRustixImpl::check_apply_write(&mut dat, param, rustix_bl::rustix_event_shop::BLEvents::SetPriceForSpecial {
                    unique_id: parsed_body.unique_id,
                    price: parsed_body.price,
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

    pub fn update_item(req: &mut iron::request::Request) -> IronResult<Response> {
        let posted_body = extract_body(req);
        println!("posted_body = {:?}", posted_body);
        let parsed_body: UpdateItem = serde_json::from_str(&posted_body).unwrap();
        let datholder = req.get::<State<SharedBackend>>().unwrap();
        let mut dat = datholder.write().unwrap();
        let query_str = extract_query(req);

        match query_str {
            Some(json_query) => {
                let param: ParametersAll = serde_json::from_str(&json_query).unwrap();

                let result = ServableRustixImpl::check_apply_write(&mut dat, param, rustix_bl::rustix_event_shop::BLEvents::UpdateItem {
                    item_id: parsed_body.item_id,
                    itemname: parsed_body.name,
                    price_cents: parsed_body.price_cents,
                    category: parsed_body.category,
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

    pub fn top_items(req: &mut iron::request::Request) -> IronResult<Response> {
        let datholder = req.get::<State<SharedBackend>>().unwrap();
        let dat = datholder.read().unwrap();
        let query_str = extract_query(req);

        match query_str {
            Some(json_query) => {
                let param: ParametersTopPersonalDrinks = serde_json::from_str(&json_query).unwrap();

                let result = ServableRustixImpl::query_read(&dat, ReadQueryParams::TopPersonalDrinks(param));

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

    pub fn get_ffa_giveouts(req: &mut iron::request::Request) -> IronResult<Response> {
        let datholder = req.get::<State<SharedBackend>>().unwrap();
        let dat = datholder.read().unwrap();
        let query_str = extract_query(req);

        match query_str {
            Some(json_query) => {
                let param: ParametersOpenFFAFreebies = serde_json::from_str(&json_query).unwrap();

                let result = ServableRustixImpl::query_read(&dat, ReadQueryParams::OpenFFAFreebies(param));

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

    pub fn get_incoming_giveouts(req: &mut iron::request::Request) -> IronResult<Response> {
        let datholder = req.get::<State<SharedBackend>>().unwrap();
        let dat = datholder.read().unwrap();
        let query_str = extract_query(req);

        match query_str {
            Some(json_query) => {
                let param: ParametersIncomingFreebies = serde_json::from_str(&json_query).unwrap();

                let result = ServableRustixImpl::query_read(&dat, ReadQueryParams::IncomingFreebies(param));

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

    pub fn get_outgoing_giveouts(req: &mut iron::request::Request) -> IronResult<Response> {
        let datholder = req.get::<State<SharedBackend>>().unwrap();
        let dat = datholder.read().unwrap();
        let query_str = extract_query(req);

        match query_str {
            Some(json_query) => {
                let param: ParametersOutgoingFreebies = serde_json::from_str(&json_query).unwrap();

                let result = ServableRustixImpl::query_read(&dat, ReadQueryParams::OutgoingFreebies(param));

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

    pub fn all_items(req: &mut iron::request::Request) -> IronResult<Response> {
        let datholder = req.get::<State<SharedBackend>>().unwrap();
        let dat = datholder.read().unwrap();
        let query_str = extract_query(req);

        match query_str {
            Some(json_query) => {
                let param: ParametersAllItems = serde_json::from_str(&json_query).unwrap();

                let result = ServableRustixImpl::query_read(&dat, ReadQueryParams::AllItems(param));

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

    pub fn user_detail_info(req: &mut iron::request::Request) -> IronResult<Response> {
        let datholder = req.get::<State<SharedBackend>>().unwrap();
        let dat = datholder.read().unwrap();
        let query_str = extract_query(req);

        match query_str {
            Some(json_query) => {
                let param: ParametersDetailInfoForUser = serde_json::from_str(&json_query).unwrap();

                let result = ServableRustixImpl::query_read(&dat, ReadQueryParams::DetailInfoForUser(param));

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

    pub fn personal_log(req: &mut iron::request::Request) -> IronResult<Response> {
        let datholder = req.get::<State<SharedBackend>>().unwrap();
        let dat = datholder.read().unwrap();
        let query_str = extract_query(req);

        match query_str {
            Some(json_query) => {
                let param: ParametersPurchaseLogPersonal = serde_json::from_str(&json_query).unwrap();

                let result = ServableRustixImpl::query_read(&dat, ReadQueryParams::PurchaseLogPersonal(param));

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

    pub fn get_bills(req: &mut iron::request::Request) -> IronResult<Response> {
        let datholder = req.get::<State<SharedBackend>>().unwrap();
        let dat = datholder.read().unwrap();
        let query_str = extract_query(req);

        match query_str {
            Some(json_query) => {
                let param: ParametersBills = serde_json::from_str(&json_query).unwrap();

                let result = ServableRustixImpl::query_read(&dat, ReadQueryParams::Bills(param));

                println!("Bills are queried with result = {:?}", result);

                match result {
                    Ok(sux) => return Ok(Response::with((iron::status::Ok, serde_json::to_string(&sux).unwrap()))),
                    Err(err) => return Ok(Response::with((iron::status::Conflict, serde_json::to_string(&PaginatedResult::<rustix_bl::datastore::Bill> {
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

    pub fn get_detailed_bill(req: &mut iron::request::Request) -> IronResult<Response> {
        let datholder = req.get::<State<SharedBackend>>().unwrap();
        let dat = datholder.read().unwrap();
        let query_str = extract_query(req);

        match query_str {
            Some(json_query) => {
                let param: ParametersBillDetails = serde_json::from_str(&json_query).unwrap();

                let result = ServableRustixImpl::query_read(&dat, ReadQueryParams::BillDetails(param));

                println!("Bill details are queried with result = {:?}", result);

                match result {
                    Ok(sux) => return Ok(Response::with((iron::status::Ok, serde_json::to_string(&sux).unwrap()))),
                    Err(err) => return Ok(Response::with((iron::status::Conflict, serde_json::to_string(&PaginatedResult::<DetailedBill> {
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

    pub fn global_log(req: &mut iron::request::Request) -> IronResult<Response> {
        let datholder = req.get::<State<SharedBackend>>().unwrap();
        let dat = datholder.read().unwrap();
        let query_str = extract_query(req);

        match query_str {
            Some(json_query) => {
                let param: ParametersPurchaseLogGlobal = serde_json::from_str(&json_query).unwrap();

                let result = ServableRustixImpl::query_read(&dat, ReadQueryParams::PurchaseLogGlobal(param));

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

    pub fn top_users(req: &mut iron::request::Request) -> IronResult<Response> {
        let datholder = req.get::<State<SharedBackend>>().unwrap();
        let dat = datholder.read().unwrap();
        let query_str = extract_query(req);

        match query_str {
            Some(json_query) => {
                let param: ParametersTopUsers = serde_json::from_str(&json_query).unwrap();

                let result = ServableRustixImpl::query_read(&dat, ReadQueryParams::TopUsers(param));

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
}


pub fn execute_cervisia_server(with_config: &ServerConfig,
                               old_server: Option<iron::Listening>, backend: Option<Backend>) -> (iron::Listening) {

    if false {
        let mut attachments : std::collections::HashMap<String, String> = std::collections::HashMap::new();

        attachments.insert("first_attachment.txt".to_string(), "Hello World from first attachment\nWith a linebreak!".to_string());
        attachments.insert("second_attachment.txt".to_string(), "Hello World from second attachment\nWith a linebreak!".to_string());


    let email = mail::send_mail("christopher.kaag@gmail.com", "my test subject line wit", "my complete body\nwith a linebreak", &attachments, with_config);
    println!("{:?}", email.unwrap());
    }








    info!("execute_cervisia_server begins for config = {:?}", with_config);

    if old_server.is_some() {
        info!("Closing old server");
        //TODO: does not work, see https://github.com/hyperium/hyper/issues/338
        old_server.unwrap().close().unwrap();
    };


    info!("Building server");

    let mut server = build_server(with_config, backend);

    println!("Having built server");


    return server;
}

#[derive(Serialize, Deserialize, Clone, Debug, TypeScriptify)]
pub struct ServerWriteResult {
    pub error_message: Option<String>,
    pub is_success: bool,
    pub content: Option<SuccessContent>,
}

#[derive(Serialize, Deserialize, Clone, Debug, TypeScriptify)]
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

#[derive(Serialize, Deserialize, Clone, Debug, TypeScriptify)]
pub struct RefreshedData {
    //TODO: use option<PaginatedResult<T>> instead for each field
    pub DetailInfoForUser: serde_json::Value,
    pub TopUsers: serde_json::Value,
    pub AllUsers: serde_json::Value,
    pub AllItems: serde_json::Value,
    pub PurchaseLogGlobal: serde_json::Value,
    pub LastPurchases: serde_json::Value,
    pub BillsCount: serde_json::Value,
    pub Bills: serde_json::Value,
    pub BillDetails: serde_json::Value,
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
            top_items_per_user: 4,
            host: "localhost".to_string(),
            server_port: get_and_increment_port(),
            web_path: "web/".to_string(),
            use_persistence: false,
            persistence_file_path: String::new(),
            use_sendmail_instead_of_smtp: None,
            sender_email_address: String::new(),
            smtp_host_address: String::new(),
            smpt_credentials_loginname: String::new(),
            smpt_credentials_password: String::new(),
            smtp_port: 0,
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
            bills: ParametersBills {
                count_pars: ParametersBillsCount {
                    start_inclusive: 0,
                    end_exclusive: 0,
                    scope_user_id: None,
                },
                pagination: ParametersPagination { start_inclusive: 0, end_exclusive: 0 },
            },
            bill_detail_infos: ParametersBillDetails { timestamp_from: None, timestamp_to: None },
            open_ffa_freebies: ParametersOpenFFAFreebies { pagination: ParametersPagination { start_inclusive: 0, end_exclusive: 0 } },
            top_personal_drinks: ParametersTopPersonalDrinks { user_id: 0, n: 0 },
            personal_log: ParametersPurchaseLogPersonal {
                count_pars: ParametersPurchaseLogPersonalCount {
                    user_id: 0,
                    millis_start: 0,
                    millis_end: 0,
                },
                pagination: ParametersPagination { start_inclusive: 0, end_exclusive: 0 },
            },
            incoming_freebies: ParametersIncomingFreebies { count_pars: ParametersIncomingFreebiesCount { recipient_id: 0 }, pagination: ParametersPagination { start_inclusive: 0, end_exclusive: 0 } },
            outgoing_freebies: ParametersOutgoingFreebies { count_pars: ParametersOutgoingFreebiesCount { donor_id: 0 }, pagination: ParametersPagination { start_inclusive: 0, end_exclusive: 0 } },
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
            bills: ParametersBills {
                count_pars: ParametersBillsCount {
                    start_inclusive: 0,
                    end_exclusive: 0,
                    scope_user_id: None,
                },
                pagination: ParametersPagination { start_inclusive: 0, end_exclusive: 0 },
            },
            bill_detail_infos: ParametersBillDetails { timestamp_from: None, timestamp_to: None },
            open_ffa_freebies: ParametersOpenFFAFreebies { pagination: ParametersPagination { start_inclusive: 0, end_exclusive: 0 } },
            top_personal_drinks: ParametersTopPersonalDrinks { user_id: 0, n: 0 },
            personal_log: ParametersPurchaseLogPersonal {
                count_pars: ParametersPurchaseLogPersonalCount {
                    user_id: 0,
                    millis_start: 0,
                    millis_end: 0,
                },
                pagination: ParametersPagination { start_inclusive: 0, end_exclusive: 0 },
            },
            incoming_freebies: ParametersIncomingFreebies { count_pars: ParametersIncomingFreebiesCount { recipient_id: 0 }, pagination: ParametersPagination { start_inclusive: 0, end_exclusive: 0 } },
            outgoing_freebies: ParametersOutgoingFreebies { count_pars: ParametersOutgoingFreebiesCount { donor_id: 0 }, pagination: ParametersPagination { start_inclusive: 0, end_exclusive: 0 } },
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