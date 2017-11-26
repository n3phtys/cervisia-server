use rustix_bl;
use rustix_bl::*;
use rustix_bl::datastore::*;
use std::vec::*;
use std::collections::*;


pub struct ParametersAll {
    pub top_users : ParametersTopUsers,
    pub all_users : ParametersAllUsers,
    pub all_items : ParametersAllItems,
    pub global_log : ParametersPurchaseLogGlobal,
    pub bills : ParametersBills,
    pub open_ffa_freebies : ParametersOpenFFAFreebies,
    pub top_personal_drinks : ParametersTopPersonalDrinks,
    pub personal_log : ParametersPurchaseLogPersonal,
    pub incoming_freebies : ParametersIncomingFreebies,
    pub outgoing_freebies : ParametersOutgoingFreebies,
    pub personal_detail_infos : ParametersDetailInfoForUser,
}

pub struct ParametersPagination {
    pub start_inclusive: u32,
    pub end_exclusive: u32,
}

pub struct ParametersTopUsers {
    searchterm: String,
    n : u16,
}

pub struct ParametersAllUsersCount {
    searchterm: String,
}

pub struct ParametersAllUsers {
    pub count_pars : ParametersAllUsersCount,
    pub pagination : ParametersPagination,
}

pub struct ParametersAllItemsCount {
    searchterm: String,
}

pub struct ParametersAllItems {
    pub count_pars : ParametersAllItemsCount,
    pub pagination : ParametersPagination,
}

pub struct ParametersPurchaseLogGlobalCount {
    pub millis_start: u64,
    pub millis_end: u64,
}

pub struct ParametersPurchaseLogGlobal{
    pub count_pars : ParametersPurchaseLogGlobalCount,
    pub pagination : ParametersPagination,

}


pub struct ParametersBillsCount {}

pub struct ParametersBills{
    pub count_pars : ParametersBillsCount,
    pub pagination : ParametersPagination,

}



pub struct ParametersOpenFFAFreebies {
//TODO: implement
}

pub struct ParametersTopPersonalDrinks {
    n : u8,
}

pub struct ParametersPurchaseLogPersonalCount {
    user_id: u64,
    millis_start: u64,
    millis_end: u64,
}

pub struct ParametersPurchaseLogPersonal {
    pub count_pars : ParametersPurchaseLogPersonalCount,
    pub pagination : ParametersPagination,

}

pub struct ParametersIncomingFreebiesCount {
//TODO: implement
}

pub struct ParametersIncomingFreebies {
//TODO: implement
}

pub struct ParametersOutgoingFreebiesCount {
//TODO: implement
}

pub struct ParametersOutgoingFreebies {
    //TODO: implement
}

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

