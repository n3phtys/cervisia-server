use rustix_bl;
use rustix_bl::*;
use rustix_bl::datastore::*;
use std::vec::*;
use server;
use server::RefreshedData;
use std::collections::*;
use serde_json;
use std;
use rustix_bl::rustix_event_shop;
use std::sync::RwLock;
use rand::{Rng, SeedableRng, StdRng};
use rustix_bl::rustix_backend::*;
use server::Backend;
use std::ops::Try;
use std::option::NoneError;




#[derive(Serialize, Deserialize)]
pub struct ParametersPagination {
    pub start_inclusive: u32,
    pub end_exclusive: u32,
}


#[derive(Serialize, Deserialize)]
pub enum ReadQueryParams {
    DetailInfoForUser(ParametersDetailInfoForUser),
    TopUsers(ParametersTopUsers),
    AllUsersCount(ParametersAllUsersCount),
    AllUsers(ParametersAllUsers),
    AllItemsCount(ParametersAllItemsCount),
    AllItems(ParametersAllItems),
    PurchaseLogGlobalCount(ParametersPurchaseLogGlobalCount),
    PurchaseLogGlobal(ParametersPurchaseLogGlobal),
    BillsCount(ParametersBillsCount),
    Bills(ParametersBills),
    OpenFFAFreebies(ParametersOpenFFAFreebies),
    TopPersonalDrinks(ParametersTopPersonalDrinks),
    PurchaseLogPersonalCount(ParametersPurchaseLogPersonalCount),
    PurchaseLogPersonal(ParametersPurchaseLogPersonal),
    IncomingFreebiesCount(ParametersIncomingFreebiesCount),
    IncomingFreebies(ParametersIncomingFreebies),
    OutgoingFreebiesCount(ParametersOutgoingFreebiesCount),
    OutgoingFreebies(ParametersOutgoingFreebies),
}



#[derive(Serialize, Deserialize)]
pub struct ParametersAll {
    pub top_users: ParametersTopUsers,
    pub all_users: ParametersAllUsers,
    pub all_items: ParametersAllItems,
    pub global_log: ParametersPurchaseLogGlobal,
    pub bills: ParametersBills,
    pub open_ffa_freebies: ParametersOpenFFAFreebies,
    pub top_personal_drinks: ParametersTopPersonalDrinks,
    pub personal_log: ParametersPurchaseLogPersonal,
    pub incoming_freebies: ParametersIncomingFreebies,
    pub outgoing_freebies: ParametersOutgoingFreebies,
    pub personal_detail_infos: ParametersDetailInfoForUser,
}


#[derive(Serialize, Deserialize)]
pub struct ParametersTopUsers {
    //decided not to use this: pub searchterm: String,
    pub n: u16,
}

#[derive(Serialize, Deserialize)]
pub struct ParametersAllUsersCount {
    pub searchterm: String,
}

#[derive(Serialize, Deserialize)]
pub struct ParametersAllUsers {
    pub count_pars: ParametersAllUsersCount,
    pub pagination: ParametersPagination,
}

#[derive(Serialize, Deserialize)]
pub struct ParametersAllItemsCount {
    pub searchterm: String,
}

#[derive(Serialize, Deserialize)]
pub struct ParametersAllItems {
    pub count_pars: ParametersAllItemsCount,
    pub pagination: ParametersPagination,
}

#[derive(Serialize, Deserialize)]
pub struct ParametersPurchaseLogGlobalCount {
    pub millis_start: i64,
    pub millis_end: i64,
}

#[derive(Serialize, Deserialize)]
pub struct ParametersPurchaseLogGlobal {
    pub count_pars: ParametersPurchaseLogGlobalCount,
    pub pagination: ParametersPagination,

}


#[derive(Serialize, Deserialize)]
pub struct ParametersBillsCount {}

#[derive(Serialize, Deserialize)]
pub struct ParametersBills {
    pub count_pars: ParametersBillsCount,
    pub pagination: ParametersPagination,

}


#[derive(Serialize, Deserialize)]
pub struct ParametersOpenFFAFreebies {
//TODO: implement
}

#[derive(Serialize, Deserialize)]
pub struct ParametersTopPersonalDrinks {
    pub user_id: u32,
    pub n: u8,
}

#[derive(Serialize, Deserialize)]
pub struct ParametersPurchaseLogPersonalCount {
    pub user_id: u32,
    pub millis_start: i64,
    pub millis_end: i64,
}

#[derive(Serialize, Deserialize)]
pub struct ParametersPurchaseLogPersonal {
    pub count_pars: ParametersPurchaseLogPersonalCount,
    pub pagination: ParametersPagination,

}

#[derive(Serialize, Deserialize)]
pub struct ParametersIncomingFreebiesCount {
//TODO: implement
}

#[derive(Serialize, Deserialize)]
pub struct ParametersIncomingFreebies {
//TODO: implement
}

#[derive(Serialize, Deserialize)]
pub struct ParametersOutgoingFreebiesCount {
//TODO: implement
}

#[derive(Serialize, Deserialize)]
pub struct ParametersOutgoingFreebies {
    //TODO: implement
}

#[derive(Serialize, Deserialize)]
pub struct ParametersDetailInfoForUser {
    pub user_id: u32,
}


pub trait RustixReadsGlobal {
    fn top_users(par: &ParametersTopUsers) -> Vec<User>;
    fn all_users_count(par: &ParametersAllUsersCount) -> u32;
    fn all_users(par: &ParametersAllUsers) -> Vec<User>;

    fn all_items_count(par: &ParametersAllItemsCount) -> u32;
    fn all_items(par: &ParametersAllItems) -> Vec<Item>;

    fn all_bills_count(par: &ParametersBillsCount) -> u32;
    fn all_bills(par: &ParametersBills) -> u32;


    fn purchase_log_global_count(par: &ParametersPurchaseLogGlobalCount) -> u32;
    fn purchase_log_global(par: &ParametersPurchaseLogGlobal) -> Vec<Purchase>;

    //fn open_ffa_items(par : &ParametersAllUsers) -> Vec<Freeby>; //TODO: not implemented in rustix yet
}

pub trait RustixReadsPersonal {
    fn top_drinks_per_user(par: &ParametersTopPersonalDrinks) -> HashMap<u64, Item>;
    fn purchase_log_personal_count(par: &ParametersPurchaseLogPersonalCount) -> u32;
    fn purchase_log_personal(par: &ParametersPurchaseLogPersonal) -> Vec<Purchase>;

    //fn freebies_incoming_count(par : &ParametersAllUsers) -> Vec<Freeby>; //TODO: not implemented in rustix yet
    //fn freebies_incoming(par : &ParametersAllUsers) -> Vec<Freeby>; //TODO: not implemented in rustix yet
    //fn freebies_outgoing_count(par : &ParametersAllUsers) -> Vec<Freeby>; //TODO: not implemented in rustix yet
    //fn freebies_outgoing(par : &ParametersAllUsers) -> Vec<Freeby>; //TODO: not implemented in rustix yet

    fn detail_info_for_user(par: &ParametersDetailInfoForUser) -> UserDetailInfo;
}

#[derive(Serialize, Deserialize)]
pub struct UserDetailInfo {
    pub consumed: HashMap<String, u32>,
    //pub open_freebies: Vec<Freeby>,
    pub last_bill_date: i64,
    pub last_bill_cost: u32,
    pub currently_cost: u32,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Purchase {
    UndoPurchase { unique_id: u64 },
    SimplePurchase {
        unique_id: u64,
        timestamp_epoch_millis: i64,
        item: rustix_bl::datastore::Item,
        consumer: rustix_bl::datastore::User,
    },
}


#[derive(Debug, Serialize, Deserialize)]
pub struct MyNoneError {

}

impl std::fmt::Display for MyNoneError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "MyNoneError()")
    }
}

impl From<NoneError> for MyNoneError {
    fn from(_: NoneError) -> Self {
        return MyNoneError {};
    }
}

impl std::error::Error for MyNoneError {
    fn description(&self) -> &str {
        return "element not found";
    }
}

pub trait ErrorUnwrap<T> {
    fn unwrap_or_error(self) -> Result<T, Box<MyNoneError>>;
}

impl<T> ErrorUnwrap<T> for Option<T> {
    fn unwrap_or_error(self) -> Result<T, Box<MyNoneError>> {
        if (self.is_some()) {
            return Ok(self.unwrap());
        } else {
            return Err(Box::new(MyNoneError{}))
        }
    }
}

fn enrich_purchase(incoming: &rustix_bl::datastore::Purchase, datastore: &rustix_bl::datastore::Datastore) -> std::result::Result<Purchase, Box<std::error::Error>> {
    return match *incoming {
        rustix_bl::datastore::Purchase::UndoPurchase{ref unique_id} => {
            Ok(Purchase::UndoPurchase {
                unique_id : *unique_id,
            })
        },
        rustix_bl::datastore::Purchase::SimplePurchase{ref unique_id, ref timestamp_epoch_millis, ref item_id, ref consumer_id} => {
            Ok(Purchase::SimplePurchase {
                unique_id: *unique_id,
                timestamp_epoch_millis: *timestamp_epoch_millis,
                item: datastore.items.get(item_id).unwrap_or_error()?.clone(),
                consumer: datastore.users.get(consumer_id).unwrap_or_error()?.clone(),
            })
        },
    }
}

pub trait RustixWrites {
    fn simple_purchase();
    fn special_purchase();
    fn freebie_purchase();
    fn ffa_purchase();

    fn create_user();
    fn delete_user();
    fn edit_user();

    fn create_item();
    fn delete_item();
    fn edit_item();

    fn create_bill();

    fn undo_purchase();
}

pub trait RustixSupport {
    fn send_bill_via_mail();
    fn send_personal_statistic_via_mail();
}


//TODO: implement this / improve signature before implementing
pub trait ServableRustix {
    /**
    returns json array or number
    */
    fn query_read(backend: &Backend, query: ReadQueryParams) -> Result<serde_json::Value, Box<::std::error::Error>>;

    /**
    returns json array or number, exactly what is to be updated (using query_read() to compute new values)
    */
    fn check_apply_write(backend: &mut Backend, app_state: ParametersAll, write_event : rustix_bl::rustix_event_shop::BLEvents) -> Result<RefreshedData, Box<::std::error::Error>>;
}



pub struct ServableRustixImpl {}

impl ServableRustix for ServableRustixImpl {
    /**
    returns PaginatedResult in case of list query, and number in case of count query
    */
    fn query_read(backend: &Backend, query: ReadQueryParams) -> Result<serde_json::Value, Box<::std::error::Error>> {
        use server::*;
        use manager::ReadQueryParams::*;
        use rustix_bl::datastore::DatastoreQueries;

        //TODO: implement bit by bit

        match query {
            AllItems(param) => {

                let xs = backend.datastore.items_searchhit_ids(&param.count_pars.searchterm);

                let mut v: Vec<rustix_bl::datastore::Item> = Vec::new();
                let total = xs.len() as u32;
                for id in xs {
                    v.push(backend.datastore.items.get(&id).unwrap().clone());
                }

                let result: PaginatedResult<rustix_bl::datastore::Item> = PaginatedResult {
                    total_count: total,
                    from: param.pagination.start_inclusive,
                    to: param.pagination.end_exclusive,
                    results: v.iter().take(param.pagination.end_exclusive as usize).skip(param.pagination.start_inclusive as usize).map(|r|r.clone()).collect(),
                };

                return Ok(serde_json::from_str(&serde_json::to_string(&result)?)?);
            },
            AllUsers(param) => {


                let xs = backend.datastore.users_searchhit_ids(&param.count_pars.searchterm);

                println!("AllUsersQuery with total store =\n{:?}\nvs\n{:?}", backend.datastore.users, xs);


                let mut v: Vec<rustix_bl::datastore::User> = Vec::new();
                let total = xs.len() as u32;
                for id in xs {
                    v.push(backend.datastore.users.get(&id).unwrap().clone());
                }

                let result: PaginatedResult<rustix_bl::datastore::User> = PaginatedResult {
                    total_count: total,
                    from: param.pagination.start_inclusive,
                    to: param.pagination.end_exclusive,
                    results: v.iter().take(param.pagination.end_exclusive as usize).skip(param.pagination.start_inclusive as usize).map(|r|r.clone()).collect(),
                };

                return Ok(serde_json::from_str(&serde_json::to_string(&result)?)?);
            },
            TopUsers(param) => {
                //TODO: this requires datastore to keep all users sorted descendingly if we want to take by n


                let mut v: Vec<rustix_bl::datastore::User> = Vec::new();
                let mut total = 0u32;

                let xs = backend.datastore.top_user_ids(param.n);
                
                for id in xs {
                    //if user.username.contains(param.count_pars.searchterm) {
                    total += 1;
                    match backend.datastore.users.get(&id) {
                        Some(user) => v.push(user.clone()),
                        None => panic!("Userkey for topuser not found in user hashmap"),
                    }
                }

                let result: PaginatedResult<rustix_bl::datastore::User> = PaginatedResult {
                    total_count: total,
                    from: 0,
                    to: total,
                    results: v.iter().map(|r|r.clone()).collect(),
                };

                return Ok(serde_json::from_str(&serde_json::to_string(&result)?)?);

            },
            DetailInfoForUser(param) => {

                let now : i64 = server::current_time_millis();
                let three_months_ago : i64 = now - (90i64 * 24i64 * 3_600_000i64);

                let bill = backend.datastore.bills.last();

                let xs = backend.datastore.personal_log_filtered(param.user_id, three_months_ago, now);

                let mut hm : HashMap<String, u32> = HashMap::new();
                let mut cost = 0u32;

                for x in xs {
                    let itemname = backend.datastore.users.get(x.get_user_id()).unwrap().username.to_string();
                    let itemname2 = itemname.to_string();
                    let oldv = hm.remove(&itemname).unwrap_or(0u32);
                    let pcost = backend.datastore.items.get(x.get_item_id()).unwrap().cost_cents;
                    hm.insert(itemname2, oldv + pcost);
                }

                let mut previouscost = 0u32;

                if bill.is_some() {
                    let allmap = bill.unwrap();
                    for (user_tuple, cost_map) in &allmap.sum_of_cost_hash_map {
                        for (item_tuple, cost) in cost_map {
                            previouscost += cost;
                        }
                    }
                }


                let result: PaginatedResult<UserDetailInfo> = PaginatedResult {
                    total_count: 1,
                    from: 0,
                    to: 1,
                    results: vec![UserDetailInfo {
                        consumed: hm,
                        last_bill_date: bill.map(|b|b.timestamp).unwrap_or(0i64),
                        last_bill_cost: previouscost,
                        currently_cost: cost,
                    }],
                };

                return Ok(serde_json::from_str(&serde_json::to_string(&result)?)?);

            },
            TopPersonalDrinks(param) => {

                let xs = backend.datastore.top_item_ids(param.user_id, param.n);

                let v : Vec<rustix_bl::datastore::Item> = xs.iter().map(|id| backend.datastore.items.get(id).unwrap().clone()).collect();


                let result: PaginatedResult<rustix_bl::datastore::Item> = PaginatedResult {
                    total_count: v.len() as u32,
                    from: 0,
                    to: v.len() as u32,
                    results: v.iter().map(|r|r.clone()).collect(),
                };

                return Ok(serde_json::from_str(&serde_json::to_string(&result)?)?);

            },
            PurchaseLogGlobal(param) => {

                let mut xs = backend.datastore.global_log_filtered(param.count_pars.millis_start, param.count_pars.millis_end).to_vec();


                xs.sort_by(|x,y| y.get_timestamp().cmp(x.get_timestamp()));

                let mut xv : Vec<Purchase> = Vec::new();




                for r in xs.iter().take(param.pagination.end_exclusive as usize).skip(param.pagination.start_inclusive as usize) {
                    xv.push(enrich_purchase(r, &backend.datastore)?);
                }



                let result: PaginatedResult<Purchase> = PaginatedResult {
                    total_count: xs.len() as u32,
                    from: param.pagination.start_inclusive,
                    to: param.pagination.end_exclusive,
                    results: xv,
                };

                return Ok(serde_json::from_str(&serde_json::to_string(&result)?)?);

            },
            PurchaseLogPersonal(param) => {

                let mut xs = backend.datastore.personal_log_filtered(param.count_pars.user_id, param.count_pars.millis_start, param.count_pars.millis_end);

                xs.sort_by(|x,y| y.get_timestamp().cmp(x.get_timestamp()));


                let mut xv : Vec<Purchase> = Vec::new();

                for r in xs.iter().take(param.pagination.end_exclusive as usize).skip(param.pagination.start_inclusive as usize) {
                    xv.push(enrich_purchase(r, &backend.datastore)?);
                }


                let result: PaginatedResult<Purchase> = PaginatedResult {
                    total_count: xs.len() as u32,
                    from: param.pagination.start_inclusive,
                    to: param.pagination.end_exclusive,
                    results: xv,
                };

                return Ok(serde_json::from_str(&serde_json::to_string(&result)?)?);

            },
            _ => unimplemented!()
        }
    }

    fn check_apply_write(backend: &mut Backend, app_state: ParametersAll, write_event: rustix_bl::rustix_event_shop::BLEvents) -> Result<RefreshedData, Box<::std::error::Error>> {
        use rustix_bl::rustix_backend::WriteBackend;
        use manager::ReadQueryParams::*;
        match write_event {
            rustix_event_shop::BLEvents::CreateUser{username} => {
                let username : String = username;
                let _ = &mut backend.create_user(username);
                //refresh only 2 values:
                //refresh all users
                let all_list = Self::query_read(&*backend, AllUsers(app_state.all_users))?;
                //refresh top users
                let top_list = Self::query_read(&*backend, TopUsers(app_state.top_users))?;
                Ok(RefreshedData{
                    DetailInfoForUser: serde_json::Value::Null,
                    TopUsers: top_list,
                    AllUsers: all_list,
                    AllItems: serde_json::Value::Null,
                    PurchaseLogGlobal: serde_json::Value::Null,
                    LastPurchases: serde_json::Value::Null,
                    BillsCount: serde_json::Value::Null,
                    Bills: serde_json::Value::Null,
                    OpenFFAFreebies: serde_json::Value::Null,
                    TopPersonalDrinks: serde_json::Value::Null,
                    PurchaseLogPersonal: serde_json::Value::Null,
                    IncomingFreebies: serde_json::Value::Null,
                    OutgoingFreebies: serde_json::Value::Null,
                })
            },
            rustix_event_shop::BLEvents::MakeSimplePurchase {user_id, item_id, timestamp} => {
                //make simple (non-ffa, non-special) purchase

                let b = &mut backend.purchase(user_id, item_id, timestamp);

                //refresh 5 values:
                //refresh top users
                let top_list = Self::query_read(&*backend, TopUsers(app_state.top_users))?;
                //refresh top items for current user
                let top_items = Self::query_read(&*backend, TopPersonalDrinks(app_state.top_personal_drinks))?;
                //refresh detailinfo for user
                let detail_info = Self::query_read(&*backend, DetailInfoForUser(app_state.personal_detail_infos))?;
                //refresh global log
                let global_log = Self::query_read(&*backend, PurchaseLogGlobal(app_state.global_log))?;
                //refresh last log
                let last_log = Self::query_read(&*backend, PurchaseLogGlobal( ParametersPurchaseLogGlobal{
                    count_pars: ParametersPurchaseLogGlobalCount { millis_start: server::current_time_millis() - (1000i64 * 60 * 60 * 24), millis_end: server::current_time_millis() + 1000i64 },
                    pagination: ParametersPagination { start_inclusive: 0, end_exclusive: 5 },
                } ))?;
                //refresh personal log
                let personal_log = Self::query_read(&*backend, PurchaseLogPersonal(app_state.personal_log))?;
                //do not refresh freebies (do that on-demand)

                //TODO: fix freebies

                Ok(RefreshedData{
                    DetailInfoForUser: detail_info,
                    TopUsers: top_list,
                    AllUsers: serde_json::Value::Null,
                    AllItems: serde_json::Value::Null,
                    PurchaseLogGlobal: global_log,
                    LastPurchases: last_log,
                    BillsCount: serde_json::Value::Null,
                    Bills: serde_json::Value::Null,
                    OpenFFAFreebies: serde_json::Value::Null,
                    TopPersonalDrinks: top_items,
                    PurchaseLogPersonal: personal_log,
                    IncomingFreebies: serde_json::Value::Null,
                    OutgoingFreebies: serde_json::Value::Null,
                })
            },
            rustix_event_shop::BLEvents::MakeShoppingCartPurchase {user_id, specials, item_ids, timestamp} => {

                let b = &mut backend.cart_purchase(user_id, specials, item_ids, timestamp);

                //refresh 5 values:
                //refresh top users
                let top_list = Self::query_read(&*backend, TopUsers(app_state.top_users))?;
                //refresh top items for current user
                let top_items = Self::query_read(&*backend, TopPersonalDrinks(app_state.top_personal_drinks))?;
                //refresh detailinfo for user
                let detail_info = Self::query_read(&*backend, DetailInfoForUser(app_state.personal_detail_infos))?;
                //refresh global log
                let global_log = Self::query_read(&*backend, PurchaseLogGlobal(app_state.global_log))?;
                //refresh last log
                let last_log = Self::query_read(&*backend, PurchaseLogGlobal( ParametersPurchaseLogGlobal{
                    count_pars: ParametersPurchaseLogGlobalCount { millis_start: server::current_time_millis() - (1000i64 * 60 * 60 * 24), millis_end: server::current_time_millis() + 1000i64 },
                    pagination: ParametersPagination { start_inclusive: 0, end_exclusive: 5 },
                } ))?;
                //refresh personal log
                let personal_log = Self::query_read(&*backend, PurchaseLogPersonal(app_state.personal_log))?;
                //do not refresh freebies (do that on-demand)

                //TODO: fix freebies

                Ok(RefreshedData{
                    DetailInfoForUser: detail_info,
                    TopUsers: top_list,
                    AllUsers: serde_json::Value::Null,
                    AllItems: serde_json::Value::Null,
                    PurchaseLogGlobal: global_log,
                    LastPurchases: last_log,
                    BillsCount: serde_json::Value::Null,
                    Bills: serde_json::Value::Null,
                    OpenFFAFreebies: serde_json::Value::Null,
                    TopPersonalDrinks: top_items,
                    PurchaseLogPersonal: personal_log,
                    IncomingFreebies: serde_json::Value::Null,
                    OutgoingFreebies: serde_json::Value::Null,
                })
            },
            _ => unimplemented!()
        }
    }
}



pub fn fill_backend_with_medium_test_data(backend: &mut Backend) -> () {
    let mut back = backend;

    (*back).create_user("Gruin".to_string());
    (*back).create_user("Vall".to_string());
    (*back).create_user("rad(i)".to_string());

    for i in 0..50 {
        (*back).create_user("GenUser #".to_string() + &i.to_string());
    }

    (*back).create_item(
        "Club Mate".to_string(),
        100,
        Some("without alcohol".to_string()),
    );
    (*back).create_item("Pils".to_string(), 95, Some("Beer".to_string()));
    (*back).create_item("Whiskey".to_string(), 1200, Some("Liquor".to_string()));
    (*back).create_item("Schirker".to_string(), 1100, Some("Liquor".to_string()));
    (*back).create_item("Kräussen".to_string(), 1100, Some("Beer".to_string()));


    let seed: &[_] = &[42];
    let mut rng: StdRng = SeedableRng::from_seed(seed);


    let mut timestamp_counter = 12345678i64;
    (*back).purchase(0, 2, timestamp_counter);

    //random purchases for the existing users
    for user_id in 0..((*back).datastore.users.len() as u32) {
        let nr_of_purchases: u32 = rng.gen_range(0u32, 5u32);
        for _ in 0..nr_of_purchases {
            timestamp_counter += 1;
            let item_id: u32 = rng.gen_range(0u32, (*back).datastore.items.len() as u32);
            (*back).purchase(user_id, item_id, timestamp_counter);
        }
    }
}

pub fn fill_backend_with_large_test_data(backend: &mut Backend) -> () {
    let mut back = backend;

    (*back).create_user("Gruin".to_string());
    (*back).create_user("Vall".to_string());
    (*back).create_user("rad(i)".to_string());

    for i in 0..500 {
        (*back).create_user("GenUser #".to_string() + &i.to_string());
    }

    (*back).create_item(
        "Club Mate".to_string(),
        100,
        Some("without alcohol".to_string()),
    );
    (*back).create_item("Pils".to_string(), 95, Some("Beer".to_string()));
    (*back).create_item("Whiskey".to_string(), 1200, Some("Liquor".to_string()));
    (*back).create_item("Schirker".to_string(), 1100, Some("Liquor".to_string()));
    (*back).create_item("Kräussen".to_string(), 1100, Some("Beer".to_string()));


    let seed: &[_] = &[42];
    let mut rng: StdRng = SeedableRng::from_seed(seed);


    let mut timestamp_counter = 12345678i64;
    (*back).purchase(0, 2, timestamp_counter);

    //random purchases for the existing users
    for user_id in 0..((*back).datastore.users.len() as u32) {
        let nr_of_purchases: u32 = rng.gen_range(0u32, 15u32);
        for _ in 0..nr_of_purchases {
            timestamp_counter += 1;
            let item_id: u32 = rng.gen_range(0u32, (*back).datastore.items.len() as u32);
            (*back).purchase(user_id, item_id, timestamp_counter);
        }
    }
}

#[cfg(test)]
pub mod tests {
    use serde_json;
    use std;
    use rustix_bl;
    use rustix_bl::rustix_event_shop;
    use std::sync::RwLock;
    use manager::ParametersAll;
    use rand::{Rng, SeedableRng, StdRng};
    use rustix_bl::rustix_backend::*;
    use server::Backend;

    pub fn fill_not(backend: &mut Backend) -> () {}
}