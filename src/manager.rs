use rustix_bl;
use rustix_bl::*;
use rustix_bl::datastore::*;
use std::vec::*;
use std::collections::*;

pub trait RustixReadsGlobal {
    fn top_users(n: u16, searchterm: &str) -> Vec<User>;
    fn all_users_count(searchterm: &str) -> u32;
    fn all_users(pagination_start: u32, pagination_end: u32, searchterm: &str) -> Vec<User>;

    fn all_items_count(searchterm: &str) -> u32;
    fn all_items(pagination_start: u32, pagination_end: u32, searchterm: &str) -> Vec<Item>;


    fn purchase_log_global_count(millis_start: u64, millis_end: u64) -> u32;
    fn purchase_log_global(pagination_start: u32, pagination_end: u32, millis_start: u64, millis_end: u64) -> Vec<Purchase>;

    //fn open_ffa_items() -> Vec<Freeby>; //TODO: not implemented in rustix yet
}

pub struct UserDetailInfo {
    pub consumed: HashMap<String, u32>,
    //pub open_freebies: Vec<Freeby>,
    pub last_bill_date: u64,
    pub last_bill_cost: u32,
    pub currently_cost: u32,
}

pub trait RustixReadsPersonal {
    fn top_drinks_per_user(n: u8) -> HashMap<u64, Item>;
    fn purchase_log_personal_count(user_id: u64, millis_start: u64, millis_end: u64) -> u32;
    fn purchase_log_personal(pagination_start: u32, pagination_end: u32, user_id: u64, millis_start: u64, millis_end: u64) -> Vec<Purchase>;

    //fn freebies_incoming_count() -> Vec<Freeby>; //TODO: not implemented in rustix yet
    //fn freebies_incoming() -> Vec<Freeby>; //TODO: not implemented in rustix yet
    //fn freebies_outgoing_count() -> Vec<Freeby>; //TODO: not implemented in rustix yet
    //fn freebies_outgoing() -> Vec<Freeby>; //TODO: not implemented in rustix yet

    fn detail_info_for_user(user_id: u64) -> UserDetailInfo;
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