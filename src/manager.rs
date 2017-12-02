use rustix_bl;
use rustix_bl::*;
use rustix_bl::datastore::*;
use std::vec::*;
use std::collections::*;



type Backend = rustix_bl::rustix_backend::RustixBackend<rustix_bl::persistencer::TransientPersister>;


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
    searchterm: String,
    n : u16,
}

#[derive(Serialize, Deserialize)]
pub struct ParametersAllUsersCount {
    pub searchterm: String,
}

#[derive(Serialize, Deserialize)]
pub struct ParametersAllUsers {
    pub count_pars : ParametersAllUsersCount,
    pub pagination : ParametersPagination,
}

#[derive(Serialize, Deserialize)]
pub struct ParametersAllItemsCount {
    searchterm: String,
}

#[derive(Serialize, Deserialize)]
pub struct ParametersAllItems {
    pub count_pars : ParametersAllItemsCount,
    pub pagination : ParametersPagination,
}

#[derive(Serialize, Deserialize)]
pub struct ParametersPurchaseLogGlobalCount {
    pub millis_start: u64,
    pub millis_end: u64,
}

#[derive(Serialize, Deserialize)]
pub struct ParametersPurchaseLogGlobal{
    pub count_pars : ParametersPurchaseLogGlobalCount,
    pub pagination : ParametersPagination,

}


#[derive(Serialize, Deserialize)]
pub struct ParametersBillsCount {}

#[derive(Serialize, Deserialize)]
pub struct ParametersBills{
    pub count_pars : ParametersBillsCount,
    pub pagination : ParametersPagination,

}



#[derive(Serialize, Deserialize)]
pub struct ParametersOpenFFAFreebies {
//TODO: implement
}

#[derive(Serialize, Deserialize)]
pub struct ParametersTopPersonalDrinks {
    n : u8,
}

#[derive(Serialize, Deserialize)]
pub struct ParametersPurchaseLogPersonalCount {
    user_id: u64,
    millis_start: u64,
    millis_end: u64,
}

#[derive(Serialize, Deserialize)]
pub struct ParametersPurchaseLogPersonal {
    pub count_pars : ParametersPurchaseLogPersonalCount,
    pub pagination : ParametersPagination,

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
    user_id: u64,
}


pub trait RustixReadsGlobal {
    fn top_users(par : &ParametersTopUsers) -> Vec<User>;
    fn all_users_count(par : &ParametersAllUsersCount) -> u32;
    fn all_users(par : &ParametersAllUsers) -> Vec<User>;

    fn all_items_count(par : &ParametersAllItemsCount) -> u32;
    fn all_items(par : &ParametersAllItems) -> Vec<Item>;

    fn all_bills_count(par : &ParametersBillsCount) -> u32;
    fn all_bills(par : &ParametersBills) -> u32;


    fn purchase_log_global_count(par : &ParametersPurchaseLogGlobalCount) -> u32;
    fn purchase_log_global(par : &ParametersPurchaseLogGlobal) -> Vec<Purchase>;

    //fn open_ffa_items(par : &ParametersAllUsers) -> Vec<Freeby>; //TODO: not implemented in rustix yet
}

pub trait RustixReadsPersonal {
    fn top_drinks_per_user(par : &ParametersTopPersonalDrinks) -> HashMap<u64, Item>;
    fn purchase_log_personal_count(par : &ParametersPurchaseLogPersonalCount) -> u32;
    fn purchase_log_personal(par : &ParametersPurchaseLogPersonal) -> Vec<Purchase>;

    //fn freebies_incoming_count(par : &ParametersAllUsers) -> Vec<Freeby>; //TODO: not implemented in rustix yet
    //fn freebies_incoming(par : &ParametersAllUsers) -> Vec<Freeby>; //TODO: not implemented in rustix yet
    //fn freebies_outgoing_count(par : &ParametersAllUsers) -> Vec<Freeby>; //TODO: not implemented in rustix yet
    //fn freebies_outgoing(par : &ParametersAllUsers) -> Vec<Freeby>; //TODO: not implemented in rustix yet

    fn detail_info_for_user(par : &ParametersDetailInfoForUser) -> UserDetailInfo;
}

#[derive(Serialize, Deserialize)]
pub struct UserDetailInfo {
    pub consumed: HashMap<String, u32>,
    //pub open_freebies: Vec<Freeby>,
    pub last_bill_date: u64,
    pub last_bill_cost: u32,
    pub currently_cost: u32,
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
    fn query_read(backend: &Backend, query: ReadQueryParams) -> Result<String, Box<::std::error::Error>>;

    /**
    returns json array or number, exactly what is to be updated (using query_read() to compute new values)
    */
    fn check_apply_write(backend: &mut Backend, app_state: ParametersAll) -> Result<String, Box<::std::error::Error>>;
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

    pub fn fill_backend_with_medium_test_data(backend : &RwLock<rustix_bl::rustix_backend::RustixBackend<rustix_bl::persistencer::TransientPersister>>) -> () {
        let mut back = backend.write().unwrap();

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

}