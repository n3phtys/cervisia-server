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
use typescriptify::TypeScriptifyTrait;

#[derive(Serialize, Deserialize, TypeScriptify)]
pub struct ParametersPagination {
    pub start_inclusive: u32,
    pub end_exclusive: u32,
}


//#[derive(Serialize, Deserialize)]
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
    BillDetails(ParametersBillDetails),
    OpenFFAFreebies(ParametersOpenFFAFreebies),
    TopPersonalDrinks(ParametersTopPersonalDrinks),
    PurchaseLogPersonalCount(ParametersPurchaseLogPersonalCount),
    PurchaseLogPersonal(ParametersPurchaseLogPersonal),
    IncomingFreebiesCount(ParametersIncomingFreebiesCount),
    IncomingFreebies(ParametersIncomingFreebies),
    OutgoingFreebiesCount(ParametersOutgoingFreebiesCount),
    OutgoingFreebies(ParametersOutgoingFreebies),
}


#[derive(Serialize, Deserialize, TypeScriptify)]
pub struct ParametersAll {
    pub top_users: ParametersTopUsers,
    pub all_users: ParametersAllUsers,
    pub all_items: ParametersAllItems,
    pub global_log: ParametersPurchaseLogGlobal,
    pub bills: ParametersBills,
    pub bill_detail_infos: ParametersBillDetails,
    pub open_ffa_freebies: ParametersOpenFFAFreebies,
    pub top_personal_drinks: ParametersTopPersonalDrinks,
    pub personal_log: ParametersPurchaseLogPersonal,
    pub incoming_freebies: ParametersIncomingFreebies,
    pub outgoing_freebies: ParametersOutgoingFreebies,
    pub personal_detail_infos: ParametersDetailInfoForUser,
}


#[derive(Serialize, Deserialize, TypeScriptify)]
pub struct ParametersBillDetails {
    pub timestamp_from: Option<i64>,
    pub timestamp_to: Option<i64>,
}


#[derive(Serialize, Deserialize, TypeScriptify)]
pub struct ParametersTopUsers {
    //decided not to use this: pub searchterm: String,
    pub n: u16,
}

#[derive(Serialize, Deserialize, TypeScriptify)]
pub struct ParametersAllUsersCount {
    pub searchterm: String,
}

#[derive(Serialize, Deserialize, TypeScriptify)]
pub struct ParametersAllUsers {
    pub count_pars: ParametersAllUsersCount,
    pub pagination: ParametersPagination,
}

#[derive(Serialize, Deserialize, TypeScriptify)]
pub struct ParametersAllItemsCount {
    pub searchterm: String,
}

#[derive(Serialize, Deserialize, TypeScriptify)]
pub struct ParametersAllItems {
    pub count_pars: ParametersAllItemsCount,
    pub pagination: ParametersPagination,
}

#[derive(Serialize, Deserialize, TypeScriptify)]
pub struct ParametersPurchaseLogGlobalCount {
    pub millis_start: i64,
    pub millis_end: i64,
}

#[derive(Serialize, Deserialize, TypeScriptify)]
pub struct ParametersPurchaseLogGlobal {
    pub count_pars: ParametersPurchaseLogGlobalCount,
    pub pagination: ParametersPagination,

}


#[derive(Serialize, Deserialize, TypeScriptify)]
pub struct ParametersBillsCount {
    pub start_inclusive: i64,
    pub end_exclusive: i64,
    pub scope_user_id: Option<u32>,
}

#[derive(Serialize, Deserialize, TypeScriptify)]
pub struct ParametersBills {
    pub count_pars: ParametersBillsCount,
    pub pagination: ParametersPagination,

}


#[derive(Serialize, Deserialize, TypeScriptify)]
pub struct ParametersOpenFFAFreebies {
    pub pagination: ParametersPagination,
}

#[derive(Serialize, Deserialize, TypeScriptify)]
pub struct ParametersTopPersonalDrinks {
    pub user_id: u32,
    pub n: u8,
}


#[derive(Serialize, Deserialize, TypeScriptify)]
pub struct ParametersPurchaseLogPersonalCount {
    pub user_id: u32,
    pub millis_start: i64,
    pub millis_end: i64,
}

#[derive(Serialize, Deserialize, TypeScriptify)]
pub struct ParametersPurchaseLogPersonal {
    pub count_pars: ParametersPurchaseLogPersonalCount,
    pub pagination: ParametersPagination,

}

#[derive(Serialize, Deserialize, TypeScriptify)]
pub struct ParametersIncomingFreebiesCount {
    pub recipient_id: u32,
}

#[derive(Serialize, Deserialize, TypeScriptify)]
pub struct ParametersIncomingFreebies {
    pub count_pars: ParametersIncomingFreebiesCount,
    pub pagination: ParametersPagination,
}

#[derive(Serialize, Deserialize, TypeScriptify)]
pub struct ParametersOutgoingFreebiesCount {
    pub donor_id: u32,
}

#[derive(Serialize, Deserialize, TypeScriptify)]
pub struct ParametersOutgoingFreebies {
    pub count_pars: ParametersOutgoingFreebiesCount,
    pub pagination: ParametersPagination,
}

#[derive(Serialize, Deserialize, TypeScriptify)]
pub struct ParametersDetailInfoForUser {
    pub user_id: u32,
}



#[derive(Serialize, Deserialize, TypeScriptify)]
pub struct UserDetailInfo {
    pub consumed: HashMap<String, u32>,
    //pub open_freebies: Vec<Freeby>,
    pub last_bill_date: i64,
    pub last_bill_cost: u32,
    pub currently_cost: u32,
}


#[derive(Debug, Serialize, Deserialize, Clone, TypeScriptify)]
pub enum Purchase {
    FFAPurchase {
        unique_id: u64,
        timestamp_epoch_millis: i64,
        item: rustix_bl::datastore::Item,
        freeby: rustix_bl::datastore::Freeby,
        donor: rustix_bl::datastore::User,
    },
    SpecialPurchase {
        unique_id: u64,
        timestamp_epoch_millis: i64,
        special_name: String,
        specialcost: Option<u32>, //set to None, set to correct value during bill finalization
        consumer: rustix_bl::datastore::User,
    },
    SimplePurchase {
        unique_id: u64,
        timestamp_epoch_millis: i64,
        item: rustix_bl::datastore::Item,
        consumer: rustix_bl::datastore::User,
    },
}


#[derive(Debug, Serialize, Deserialize)]
pub struct MyNoneError {}

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
            return Err(Box::new(MyNoneError {}));
        }
    }
}

fn enrich_purchase(incoming: &rustix_bl::datastore::Purchase, datastore: &rustix_bl::datastore::Datastore) -> std::result::Result<Purchase, Box<std::error::Error>> {
    return match *incoming {
        rustix_bl::datastore::Purchase::SimplePurchase { ref unique_id, ref timestamp_epoch_millis, ref item_id, ref consumer_id } => {
            Ok(Purchase::SimplePurchase {
                unique_id: *unique_id,
                timestamp_epoch_millis: *timestamp_epoch_millis,
                item: datastore.items.get(item_id).unwrap_or_error()?.clone(),
                consumer: datastore.users.get(consumer_id).unwrap_or_error()?.clone(),
            })
        },
        rustix_bl::datastore::Purchase::SpecialPurchase { ref unique_id, ref timestamp_epoch_millis, ref special_name, ref specialcost, ref consumer_id } => {
            Ok(Purchase::SpecialPurchase {
                unique_id: *unique_id,
                timestamp_epoch_millis: *timestamp_epoch_millis,
                special_name: special_name.to_string(),
                specialcost: *specialcost,
                consumer: datastore.users.get(consumer_id).unwrap_or_error()?.clone(),
            })
        },
        rustix_bl::datastore::Purchase::FFAPurchase { ref unique_id, ref timestamp_epoch_millis, ref item_id, ref freeby_id, ref donor } => {
            Ok(Purchase::FFAPurchase {
                unique_id: *unique_id,
                timestamp_epoch_millis: *timestamp_epoch_millis,
                item: datastore.items.get(item_id).unwrap_or_error()?.clone(),
                freeby: datastore.get_ffa_freeby(*freeby_id).unwrap().clone(),
                donor: datastore.users.get(donor).unwrap_or_error()?.clone(),
            })
        },
    };
}

#[derive(Debug, Serialize, Deserialize, Clone, TypeScriptify)]
pub struct EnrichedFFA {
    pub id: u64,
    pub items: Vec<rustix_bl::datastore::Item>,
    pub total: u16,
    pub left: u16,
    pub text_message: String,
    pub created_timestamp: i64,
    pub donor: rustix_bl::datastore::User,
}

fn enrich_ffa(incoming: &rustix_bl::datastore::Freeby, datastore: &rustix_bl::datastore::Datastore) -> std::result::Result<EnrichedFFA, Box<std::error::Error>> {
    return match *incoming {
        rustix_bl::datastore::Freeby::FFA { ref id, ref allowed_categories, ref allowed_drinks, ref allowed_number_total, ref allowed_number_used, ref text_message, ref created_timestamp, ref donor } => {
            let mut items: Vec<rustix_bl::datastore::Item> = Vec::new();
            let ids: HashSet<&u32> = allowed_drinks.iter().collect();
            let cats: HashSet<String> = allowed_categories.iter().map(|s|s.to_string()).collect();
            for (_, it) in &datastore.items {
                if (!it.deleted) && (ids.contains(&it.item_id) || (it.category.is_some() && cats.contains(&it.category.clone().unwrap()))) {
                    items.push(it.clone());
                }
            }
            Ok(EnrichedFFA {
                id: *id,
                items: items,
                total: *allowed_number_total,
                left: (allowed_number_total - allowed_number_used),
                text_message: text_message.to_string(),
                created_timestamp: *created_timestamp,
                donor: datastore.users.get(donor).unwrap().clone(),
            })
        }
        _ => panic!("enrich_ffa on non-FFA called")
    };
}

#[derive(Debug, Serialize, Deserialize, Clone, TypeScriptify)]
pub struct DetailedBill {
    pub timestamp_from: i64,
    pub timestamp_to: i64,
    pub bill_state: BillState,
    pub comment: String,
    pub users: UserGroup,
    pub ready_for_finalization: bool,
    //following infos (compute by scanning over all purchases in the given area and filtering by usergroup)
    //all specials in given time and usergroup
    pub all_specials: Vec<Purchase>,
    //all special idx which are not yet set (info field, has to become empty)
    pub unset_specials_indices: Vec<usize>,
    //all users who are in this time and usergroup (base for UI selection of internal user exclusion)
    pub touched_users: Vec<rustix_bl::datastore::User>,
    //all user idxs who are excluded 'externally' (info field)
    pub users_excluded_externally_indices: Vec<usize>,
    //all user idxs who aren't excluded externally and have no external_id set (info field, has to become empty)
    pub users_undefined_indices: Vec<usize>,
    //all user idxs who are excluded 'internally' (target for UI selection of internal user exclusion)
    pub users_excluded_internally_indices: Vec<usize>,
    pub users_excludable_but_not_internally_indices: Vec<usize>,
}



pub trait ServableRustix {
    /**
    returns json array or number
    */
    fn query_read(backend: &Backend, query: ReadQueryParams) -> Result<serde_json::Value, Box<::std::error::Error>>;

    /**
    returns json array or number, exactly what is to be updated (using query_read() to compute new values)
    */
    fn check_apply_write(backend: &mut Backend, app_state: ParametersAll, write_event: rustix_bl::rustix_event_shop::BLEvents) -> Result<RefreshedData, Box<::std::error::Error>>;
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

        match query {
            AllItems(param) => {
                let xs = backend.datastore.items_searchhit_ids(&param.count_pars.searchterm);

                let mut v: Vec<rustix_bl::datastore::Item> = Vec::new();
                let total = xs.len() as u32;
                for id in xs {
                    if backend.datastore.items.contains_key(&id) {
                        v.push(backend.datastore.items.get(&id).unwrap().clone());
                    }
                }

                let result: PaginatedResult<rustix_bl::datastore::Item> = PaginatedResult {
                    total_count: total,
                    from: param.pagination.start_inclusive,
                    to: param.pagination.end_exclusive,
                    results: v.iter().take(param.pagination.end_exclusive as usize).skip(param.pagination.start_inclusive as usize).map(|r| r.clone()).collect(),
                };

                return Ok(serde_json::from_str(&serde_json::to_string(&result)?)?);
            },
            AllUsers(param) => {
                let xs = backend.datastore.users_searchhit_ids(&param.count_pars.searchterm);

                println!("AllUsersQuery with total store =\n{:?}\nvs\n{:?}", backend.datastore.users, xs);


                let mut v: Vec<rustix_bl::datastore::User> = Vec::new();
                let total = xs.len() as u32;
                for id in xs {
                    if backend.datastore.users.get(&id).is_some() {
                        v.push(backend.datastore.users.get(&id).unwrap().clone());
                    }
                }

                let result: PaginatedResult<rustix_bl::datastore::User> = PaginatedResult {
                    total_count: total,
                    from: param.pagination.start_inclusive,
                    to: param.pagination.end_exclusive,
                    results: v.iter().take(param.pagination.end_exclusive as usize).skip(param.pagination.start_inclusive as usize).map(|r| r.clone()).collect(),
                };

                return Ok(serde_json::from_str(&serde_json::to_string(&result)?)?);
            },
            TopUsers(param) => {
                //TODO: this requires datastore to keep all users sorted descendingly if we want to take by n


                let mut v: Vec<rustix_bl::datastore::User> = Vec::new();
                let mut total = 0u32;


                let highlight_users: HashSet<u32> = backend.datastore.highlighted_users.iter().map(|c| *c).collect();
                let highlighted: u16 = highlight_users.len() as u16;


                let xs_full = backend.datastore.top_user_ids(param.n);

                let mut xs : Vec<u32> = Vec::new();

                for uid in xs_full {
                    if !highlight_users.contains(&uid) && (param.n - highlighted - (xs.len() as u16)) > 0 {
                        xs.push(uid);
                    }
                }






                for id in xs {
                    //if user.username.contains(param.count_pars.searchterm) {
                    total += 1;
                    match backend.datastore.users.get(&id) {
                        Some(user) => v.push(user.clone()),
                        None => panic!("Userkey for topuser not found in user hashmap"),
                    }
                }

                for id in highlight_users {
                    total += 1;
                    match backend.datastore.users.get(&id) {
                        Some(user) => v.push(user.clone()),
                        None => panic!("Userkey for highlighted user not found in user hashmap"),
                    }
                }

                v.sort_unstable_by(|a, b| a.username.cmp(&b.username));

                let result: PaginatedResult<rustix_bl::datastore::User> = PaginatedResult {
                    total_count: total,
                    from: 0,
                    to: total,
                    results: v,
                };

                return Ok(serde_json::from_str(&serde_json::to_string(&result)?)?);
            }
            DetailInfoForUser(param) => {
                let now: i64 = server::current_time_millis();
                let three_months_ago: i64 = now - (90i64 * 24i64 * 3_600_000i64);

                let bill = backend.datastore.bills.iter().filter(|b|b.bill_state.is_finalized()).last();

                let xs = backend.datastore.personal_log_filtered(param.user_id, three_months_ago, now);

                let mut hm: HashMap<String, u32> = HashMap::new();
                let mut cost = 0u32;

                for x in xs {
                    if backend.datastore.users.get(x.get_user_id()).is_some() && x.has_item_id() && backend.datastore.items.get(x.get_item_id()).is_some() {
                        let itemname = backend.datastore.users.get(x.get_user_id()).unwrap().username.to_string();
                        let itemname2 = itemname.to_string();
                        let oldv = hm.remove(&itemname).unwrap_or(0u32);
                        let pcost = backend.datastore.items.get(x.get_item_id()).unwrap().cost_cents;
                        hm.insert(itemname2, oldv + pcost);
                    }
                }

                let mut previouscost = 0u32;

                if bill.is_some() {
                    let allmap = bill.unwrap();
                    match allmap.finalized_data.user_consumption.get(&param.user_id) {
                        Some(billuserinstance) => {
                            for (day, dayinstance) in &billuserinstance.per_day {
                                for (item_id, count) in &dayinstance.personally_consumed {
                                    let cost_once : u32 = allmap.finalized_data.all_items.get(&item_id).unwrap().cost_cents;
                                    previouscost += (cost_once * item_id);
                                }
                            }
                        },
                        None => (),
                    }
                }


                let result: PaginatedResult<UserDetailInfo> = PaginatedResult {
                    total_count: 1,
                    from: 0,
                    to: 1,
                    results: vec![UserDetailInfo {
                        consumed: hm,
                        last_bill_date: bill.map(|b| b.timestamp_to).unwrap_or(0i64),
                        last_bill_cost: previouscost,
                        currently_cost: cost,
                    }],
                };

                return Ok(serde_json::from_str(&serde_json::to_string(&result)?)?);
            },
            TopPersonalDrinks(param) => {
                let xs = backend.datastore.top_item_ids(param.user_id, param.n);

                let v: Vec<rustix_bl::datastore::Item> = xs.iter().map(|id| backend.datastore.items.get(id).unwrap().clone()).collect();


                let result: PaginatedResult<rustix_bl::datastore::Item> = PaginatedResult {
                    total_count: v.len() as u32,
                    from: 0,
                    to: v.len() as u32,
                    results: v.iter().map(|r| r.clone()).collect(),
                };

                return Ok(serde_json::from_str(&serde_json::to_string(&result)?)?);
            },
            PurchaseLogGlobal(param) => {
                let mut xs = backend.datastore.global_log_filtered(param.count_pars.millis_start, param.count_pars.millis_end).to_vec();


                xs.sort_by(|x, y| y.get_timestamp().cmp(x.get_timestamp()));

                let mut xv: Vec<Purchase> = Vec::new();


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

                xs.sort_by(|x, y| y.get_timestamp().cmp(x.get_timestamp()));


                let mut xv: Vec<Purchase> = Vec::new();

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
            Bills(param) => {
                let mut xs = backend.datastore.bills_filtered(param.count_pars.scope_user_id, param.count_pars.start_inclusive, param.count_pars.end_exclusive).to_vec();


                xs.sort_by(|x, y| y.timestamp_to.cmp(&x.timestamp_to));


                let result: PaginatedResult<Bill> = PaginatedResult {
                    total_count: xs.len() as u32,
                    from: param.pagination.start_inclusive,
                    to: param.pagination.end_exclusive,
                    results: xs,
                };

                println!("Serializing");

                let a = &serde_json::to_string(&result)?;

                println!("Result a = {:?}", a);

                let b = serde_json::from_str(a)?;

                println!("Result b = {:?}", b);

                return Ok(b);
            },
            BillDetails(param) => {
                let result = if param.timestamp_from.is_some() && param.timestamp_to.is_some() {

                    let ts_from = param.timestamp_from.unwrap();
                    let ts_to = param.timestamp_to.unwrap();

                    let bill: Bill = {
                        backend.datastore.get_bill(ts_from, ts_to).unwrap_or_error()?.clone()
                    };


                    let mut specials: Vec<Purchase> = vec![];
                    let mut unset_specials_indices: Vec<usize> = vec![];
                    let mut touched_users_set: HashSet<u32> = HashSet::new();
                    let mut touched_users: Vec<rustix_bl::datastore::User> = vec![];
                    let mut users_excluded_externally_indices: Vec<usize> = vec![];
                    let mut users_undefined_indices: Vec<usize> = vec![];
                    let mut users_excluded_internally_indices: Vec<usize> = vec![];
                    let mut users_excludable_but_not_internally_indices: Vec<usize> = vec![];


                    for purchase in backend.datastore.global_log_filtered(ts_from, ts_to) {
                        let uid: u32 = *purchase.get_user_id();
                        if matches_usergroup(&Some(uid), &bill.users) {
                            if !touched_users_set.contains(&uid) {
                                //user matches criteria & isn't in list => add user to list
                                touched_users_set.insert(uid);
                                let usr = backend.datastore.users.get(&uid).unwrap_or_error()?;
                                touched_users.push(usr.clone());

                                let user_idx: usize = touched_users.len() - 1;

                                if !usr.is_billed {
                                    //if user isn't billed per field, add to externally excluded list
                                    users_excluded_externally_indices.push(user_idx);
                                } else if bill.users_that_will_not_be_billed.contains(&uid) {
                                //else if user is in internal exclusion list of bill, add to internally excluded list
                                    users_excluded_internally_indices.push(user_idx);
                            } else if usr.external_user_id.is_none() {
                                // else add user to other list
                                    users_undefined_indices.push(user_idx);
                                    users_excludable_but_not_internally_indices.push(user_idx);
                            } else if usr.external_user_id.is_some() {
                                    users_excludable_but_not_internally_indices.push(user_idx);
                                }
                            }
                            if !purchase.has_item_id() {
                                //if special => move to special vec (if user matches)
                                specials.push(enrich_purchase(purchase, &backend.datastore)?);

                                if purchase.get_special_set_price().is_none() {
                                    //if special && unset price => move to unset special vec (if user matches)
                                    unset_specials_indices.push(specials.len() - 1);
                                }
                            }

                        }
                    }


                    let xs = vec![
                        DetailedBill {
                            timestamp_from: ts_from,
                            timestamp_to: ts_to,
                            bill_state: bill.bill_state,
                            comment: bill.comment,
                            users: bill.users,
                            ready_for_finalization: unset_specials_indices.is_empty() && users_undefined_indices.is_empty(),
                            all_specials: specials,
                            unset_specials_indices: unset_specials_indices,
                            touched_users: touched_users,
                            users_excluded_externally_indices: users_excluded_externally_indices,
                            users_undefined_indices: users_undefined_indices,
                            users_excluded_internally_indices: users_excluded_internally_indices,
                            users_excludable_but_not_internally_indices: users_excludable_but_not_internally_indices,
                        }];

                    let res: PaginatedResult<DetailedBill> = PaginatedResult {
                        total_count: xs.len() as u32,
                        from: 0,
                        to: xs.len() as u32,
                        results: xs,
                    };
                    res
                } else {
                    let res: PaginatedResult<DetailedBill> = PaginatedResult {
                        total_count: 0,
                        from: 0,
                        to: 0,
                        results: vec![],
                    };
                    res
                };

                println!("Serializing");

                let a = &serde_json::to_string(&result)?;

                println!("Result a = {:?}", a);

                let b = serde_json::from_str(a)?;

                println!("Result b = {:?}", b);

                return Ok(b);
            },
            AllUsersCount(param) => {
                panic!("Not supported")
            },
            AllItemsCount(param) => {
                panic!("Not supported")
            },
            PurchaseLogGlobalCount(param) => {
                panic!("Not supported")
            },
            BillsCount(param) => {
                panic!("Not supported")
            },
            OpenFFAFreebies(param) => {
                let xs: Vec<Freeby> = backend.datastore.open_ffa.iter().map(|x|x.clone()).collect();

                let mut xv : Vec<EnrichedFFA> = Vec::new();

                for ffa in xs {
                    xv.push(enrich_ffa(&ffa, &backend.datastore)?);
                }

                let result: PaginatedResult<EnrichedFFA> = PaginatedResult {
                    total_count: backend.datastore.open_ffa.len() as u32,
                    from: param.pagination.start_inclusive,
                    to: param.pagination.end_exclusive,
                    results: xv,
                };
                return Ok(serde_json::from_str(&serde_json::to_string(&result)?)?);
            },
            PurchaseLogPersonalCount(param) => {
                panic!("Not supported")
            },
            IncomingFreebiesCount(param) => {
                panic!("Not supported")
            },
            IncomingFreebies(param) => {
                let xsopt = backend.datastore.open_freebies.get(&param.count_pars.recipient_id);
                let emptyvec: Vec<Freeby> = Vec::new();
                let xs : &Vec<Freeby> = xsopt.unwrap_or(&emptyvec);
                let result: PaginatedResult<Freeby> = PaginatedResult {
                    total_count: xs.len() as u32,
                    from: param.pagination.start_inclusive,
                    to: param.pagination.end_exclusive,
                    results: xs.iter().take(param.pagination.end_exclusive as usize).skip(param.pagination.start_inclusive as usize).map(|r| r.clone()).collect(),
                };
                return Ok(serde_json::from_str(&serde_json::to_string(&result)?)?);
            },
            OutgoingFreebiesCount(param) => {
                panic!("Not supported")
            },
            OutgoingFreebies(param) => {
                let mut xs: Vec<Freeby> = Vec::new();

                for (key, value) in &backend.datastore.open_freebies {
                    for fb in value {
                        if fb.get_donor() == param.count_pars.donor_id {
                            xs.push(fb.clone());
                        }
                    }
                }

                let result: PaginatedResult<Freeby> = PaginatedResult {
                    total_count: xs.len() as u32,
                    from: param.pagination.start_inclusive,
                    to: param.pagination.end_exclusive,
                    results: xs.iter().take(param.pagination.end_exclusive as usize).skip(param.pagination.start_inclusive as usize).map(|r| r.clone()).collect(),
                };
                return Ok(serde_json::from_str(&serde_json::to_string(&result)?)?);
            },
        }
    }

    fn check_apply_write(backend: &mut Backend, app_state: ParametersAll, write_event: rustix_bl::rustix_event_shop::BLEvents) -> Result<RefreshedData, Box<::std::error::Error>> {
        use rustix_bl::rustix_backend::WriteBackend;
        use manager::ReadQueryParams::*;
        match write_event {
            rustix_event_shop::BLEvents::CreateUser { username } => {
                let username: String = username;
                let _ = &mut backend.create_user(username);
                //refresh only 2 values:
                //refresh all users
                let all_list = Self::query_read(&*backend, AllUsers(app_state.all_users))?;
                //refresh top users
                let top_list = Self::query_read(&*backend, TopUsers(app_state.top_users))?;
                Ok(RefreshedData {
                    DetailInfoForUser: serde_json::Value::Null,
                    TopUsers: top_list,
                    AllUsers: all_list,
                    AllItems: serde_json::Value::Null,
                    PurchaseLogGlobal: serde_json::Value::Null,
                    LastPurchases: serde_json::Value::Null,
                    BillsCount: serde_json::Value::Null,
                    Bills: serde_json::Value::Null,
                    BillDetails: serde_json::Value::Null,
                    OpenFFAFreebies: serde_json::Value::Null,
                    TopPersonalDrinks: serde_json::Value::Null,
                    PurchaseLogPersonal: serde_json::Value::Null,
                    IncomingFreebies: serde_json::Value::Null,
                    OutgoingFreebies: serde_json::Value::Null,
                })
            }
            rustix_event_shop::BLEvents::CreateItem {
                itemname,
                price_cents,
                category,
            } => {
                let _ = &mut backend.create_item(itemname, price_cents, category);

                let all_list = Self::query_read(&*backend, AllItems(app_state.all_items))?;

                Ok(RefreshedData {
                    DetailInfoForUser: serde_json::Value::Null,
                    TopUsers: serde_json::Value::Null,
                    AllUsers: serde_json::Value::Null,
                    AllItems: all_list,
                    PurchaseLogGlobal: serde_json::Value::Null,
                    LastPurchases: serde_json::Value::Null,
                    BillsCount: serde_json::Value::Null,
                    Bills: serde_json::Value::Null,
                    BillDetails: serde_json::Value::Null,
                    OpenFFAFreebies: serde_json::Value::Null,
                    TopPersonalDrinks: serde_json::Value::Null,
                    PurchaseLogPersonal: serde_json::Value::Null,
                    IncomingFreebies: serde_json::Value::Null,
                    OutgoingFreebies: serde_json::Value::Null,
                })
            }
            rustix_event_shop::BLEvents::UpdateUser { user_id, username, is_billed, is_highlighted, external_user_id } => {
                let username: String = username;
                let _ = &mut backend.update_user(user_id, username, is_billed, is_highlighted, external_user_id);
                //refresh only 2 values:
                //refresh all users
                let all_list = Self::query_read(&*backend, AllUsers(app_state.all_users))?;
                //refresh top users
                let top_list = Self::query_read(&*backend, TopUsers(app_state.top_users))?;

                let changed_bill_details = Self::query_read(&*backend, BillDetails(app_state.bill_detail_infos))?;

                Ok(RefreshedData {
                    DetailInfoForUser: serde_json::Value::Null,
                    TopUsers: top_list,
                    AllUsers: all_list,
                    AllItems: serde_json::Value::Null,
                    PurchaseLogGlobal: serde_json::Value::Null,
                    LastPurchases: serde_json::Value::Null,
                    BillsCount: serde_json::Value::Null,
                    Bills: serde_json::Value::Null,
                    BillDetails: changed_bill_details,
                    OpenFFAFreebies: serde_json::Value::Null,
                    TopPersonalDrinks: serde_json::Value::Null,
                    PurchaseLogPersonal: serde_json::Value::Null,
                    IncomingFreebies: serde_json::Value::Null,
                    OutgoingFreebies: serde_json::Value::Null,
                })
            }
            rustix_event_shop::BLEvents::UpdateItem { item_id, itemname, price_cents, category } => {
                let _ = &mut backend.update_item(item_id, itemname, price_cents, category);

                let all_list = Self::query_read(&*backend, AllItems(app_state.all_items))?;

                Ok(RefreshedData {
                    DetailInfoForUser: serde_json::Value::Null,
                    TopUsers: serde_json::Value::Null,
                    AllUsers: serde_json::Value::Null,
                    AllItems: all_list,
                    PurchaseLogGlobal: serde_json::Value::Null,
                    LastPurchases: serde_json::Value::Null,
                    BillsCount: serde_json::Value::Null,
                    Bills: serde_json::Value::Null,
                    BillDetails: serde_json::Value::Null,
                    OpenFFAFreebies: serde_json::Value::Null,
                    TopPersonalDrinks: serde_json::Value::Null,
                    PurchaseLogPersonal: serde_json::Value::Null,
                    IncomingFreebies: serde_json::Value::Null,
                    OutgoingFreebies: serde_json::Value::Null,
                })
            }
            rustix_event_shop::BLEvents::DeleteUser { user_id } => {
                let _ = &mut backend.delete_user(user_id);
                //refresh only 2 values:
                //refresh all users
                let all_list = Self::query_read(&*backend, AllUsers(app_state.all_users))?;
                //refresh top users
                let top_list = Self::query_read(&*backend, TopUsers(app_state.top_users))?;

                let changed_bill_details = Self::query_read(&*backend, BillDetails(app_state.bill_detail_infos))?;

                Ok(RefreshedData {
                    DetailInfoForUser: serde_json::Value::Null,
                    TopUsers: top_list,
                    AllUsers: all_list,
                    AllItems: serde_json::Value::Null,
                    PurchaseLogGlobal: serde_json::Value::Null,
                    LastPurchases: serde_json::Value::Null,
                    BillsCount: serde_json::Value::Null,
                    Bills: serde_json::Value::Null,
                    BillDetails: changed_bill_details,
                    OpenFFAFreebies: serde_json::Value::Null,
                    TopPersonalDrinks: serde_json::Value::Null,
                    PurchaseLogPersonal: serde_json::Value::Null,
                    IncomingFreebies: serde_json::Value::Null,
                    OutgoingFreebies: serde_json::Value::Null,
                })
            }
            rustix_event_shop::BLEvents::DeleteItem { item_id } => {
                let _ = &mut backend.delete_item(item_id);

                let all_list = Self::query_read(&*backend, AllItems(app_state.all_items))?;

                Ok(RefreshedData {
                    DetailInfoForUser: serde_json::Value::Null,
                    TopUsers: serde_json::Value::Null,
                    AllUsers: serde_json::Value::Null,
                    AllItems: all_list,
                    PurchaseLogGlobal: serde_json::Value::Null,
                    LastPurchases: serde_json::Value::Null,
                    BillsCount: serde_json::Value::Null,
                    Bills: serde_json::Value::Null,
                    BillDetails: serde_json::Value::Null,
                    OpenFFAFreebies: serde_json::Value::Null,
                    TopPersonalDrinks: serde_json::Value::Null,
                    PurchaseLogPersonal: serde_json::Value::Null,
                    IncomingFreebies: serde_json::Value::Null,
                    OutgoingFreebies: serde_json::Value::Null,
                })
            }
            rustix_event_shop::BLEvents::MakeSimplePurchase { user_id, item_id, timestamp } => {
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
                let last_log = Self::query_read(&*backend, PurchaseLogGlobal(ParametersPurchaseLogGlobal {
                    count_pars: ParametersPurchaseLogGlobalCount { millis_start: server::current_time_millis() - (1000i64 * 60 * 60 * 24), millis_end: server::current_time_millis() + 1000i64 },
                    pagination: ParametersPagination { start_inclusive: 0, end_exclusive: 5 },
                }))?;
                //refresh personal log
                let personal_log = Self::query_read(&*backend, PurchaseLogPersonal(app_state.personal_log))?;
                //do not refresh freebies (do that on-demand)

                //TODO: fix freebies

                Ok(RefreshedData {
                    DetailInfoForUser: detail_info,
                    TopUsers: top_list,
                    AllUsers: serde_json::Value::Null,
                    AllItems: serde_json::Value::Null,
                    PurchaseLogGlobal: global_log,
                    LastPurchases: last_log,
                    BillsCount: serde_json::Value::Null,
                    Bills: serde_json::Value::Null,
                    BillDetails: serde_json::Value::Null,
                    OpenFFAFreebies: serde_json::Value::Null,
                    TopPersonalDrinks: top_items,
                    PurchaseLogPersonal: personal_log,
                    IncomingFreebies: serde_json::Value::Null,
                    OutgoingFreebies: serde_json::Value::Null,
                })
            }
            rustix_event_shop::BLEvents::UndoPurchase { unique_id } => {
                //make simple (non-ffa, non-special) purchase

                let b = &mut backend.undo_purchase(unique_id);

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
                let last_log = Self::query_read(&*backend, PurchaseLogGlobal(ParametersPurchaseLogGlobal {
                    count_pars: ParametersPurchaseLogGlobalCount { millis_start: server::current_time_millis() - (1000i64 * 60 * 60 * 24), millis_end: server::current_time_millis() + 1000i64 },
                    pagination: ParametersPagination { start_inclusive: 0, end_exclusive: 5 },
                }))?;
                //refresh personal log
                let personal_log = Self::query_read(&*backend, PurchaseLogPersonal(app_state.personal_log))?;
                //do not refresh freebies (do that on-demand)

                //TODO: fix freebies

                Ok(RefreshedData {
                    DetailInfoForUser: detail_info,
                    TopUsers: top_list,
                    AllUsers: serde_json::Value::Null,
                    AllItems: serde_json::Value::Null,
                    PurchaseLogGlobal: global_log,
                    LastPurchases: last_log,
                    BillsCount: serde_json::Value::Null,
                    Bills: serde_json::Value::Null,
                    BillDetails: serde_json::Value::Null,
                    OpenFFAFreebies: serde_json::Value::Null,
                    TopPersonalDrinks: top_items,
                    PurchaseLogPersonal: personal_log,
                    IncomingFreebies: serde_json::Value::Null,
                    OutgoingFreebies: serde_json::Value::Null,
                })
            },
            a @ rustix_event_shop::BLEvents::MakeFreeForAllPurchase { .. } => {
                let _b = &mut backend.apply(&a);
                //refresh global log
                //refresh lastpurchase
                //open ffa freebies
                //refresh global log
                let global_log = Self::query_read(&*backend, PurchaseLogGlobal(app_state.global_log))?;
                //refresh last log
                let last_log = Self::query_read(&*backend, PurchaseLogGlobal(ParametersPurchaseLogGlobal {
                    count_pars: ParametersPurchaseLogGlobalCount { millis_start: server::current_time_millis() - (1000i64 * 60 * 60 * 24), millis_end: server::current_time_millis() + 1000i64 },
                    pagination: ParametersPagination { start_inclusive: 0, end_exclusive: 5 },
                }))?;
                let open_ffa = Self::query_read(&*backend, OpenFFAFreebies(app_state.open_ffa_freebies))?;

                Ok(RefreshedData {
                    DetailInfoForUser: serde_json::Value::Null,
                    TopUsers: serde_json::Value::Null,
                    AllUsers: serde_json::Value::Null,
                    AllItems: serde_json::Value::Null,
                    PurchaseLogGlobal: global_log,
                    LastPurchases: last_log,
                    BillsCount: serde_json::Value::Null,
                    Bills: serde_json::Value::Null,
                    BillDetails: serde_json::Value::Null,
                    OpenFFAFreebies: open_ffa,
                    TopPersonalDrinks: serde_json::Value::Null,
                    PurchaseLogPersonal: serde_json::Value::Null,
                    IncomingFreebies: serde_json::Value::Null,
                    OutgoingFreebies: serde_json::Value::Null,
                })
            },
            a @ rustix_event_shop::BLEvents::CreateFreeForAll { .. } => {
                let _b = &mut backend.apply(&a);
                //refresh ffa
                let open_ffa = Self::query_read(&*backend, OpenFFAFreebies(app_state.open_ffa_freebies))?;
                Ok(RefreshedData {
                    DetailInfoForUser: serde_json::Value::Null,
                    TopUsers: serde_json::Value::Null,
                    AllUsers: serde_json::Value::Null,
                    AllItems: serde_json::Value::Null,
                    PurchaseLogGlobal: serde_json::Value::Null,
                    LastPurchases: serde_json::Value::Null,
                    BillsCount: serde_json::Value::Null,
                    Bills: serde_json::Value::Null,
                    BillDetails: serde_json::Value::Null,
                    OpenFFAFreebies: open_ffa,
                    TopPersonalDrinks: serde_json::Value::Null,
                    PurchaseLogPersonal: serde_json::Value::Null,
                    IncomingFreebies: serde_json::Value::Null,
                    OutgoingFreebies: serde_json::Value::Null,
                })
            },
            a @ rustix_event_shop::BLEvents::CreateFreeCount { .. } => {
                let _b = &mut backend.apply(&a);
                //refresh incoming freebies
                //refresh outgoing freebies
                let incoming = Self::query_read(&*backend, IncomingFreebies(app_state.incoming_freebies))?;
                let outgoing = Self::query_read(&*backend, OutgoingFreebies(app_state.outgoing_freebies))?;
                Ok(RefreshedData {
                    DetailInfoForUser: serde_json::Value::Null,
                    TopUsers: serde_json::Value::Null,
                    AllUsers: serde_json::Value::Null,
                    AllItems: serde_json::Value::Null,
                    PurchaseLogGlobal: serde_json::Value::Null,
                    LastPurchases: serde_json::Value::Null,
                    BillsCount: serde_json::Value::Null,
                    Bills: serde_json::Value::Null,
                    BillDetails: serde_json::Value::Null,
                    OpenFFAFreebies: serde_json::Value::Null,
                    TopPersonalDrinks: serde_json::Value::Null,
                    PurchaseLogPersonal: serde_json::Value::Null,
                    IncomingFreebies: incoming,
                    OutgoingFreebies: outgoing,
                })
            },
            a @ rustix_event_shop::BLEvents::CreateFreeBudget { .. } => {
                let _b = &mut backend.apply(&a);
                //refresh incoming freebies
                //refresh outgoing freebies
                let incoming = Self::query_read(&*backend, IncomingFreebies(app_state.incoming_freebies))?;
                let outgoing = Self::query_read(&*backend, OutgoingFreebies(app_state.outgoing_freebies))?;
                Ok(RefreshedData {
                    DetailInfoForUser: serde_json::Value::Null,
                    TopUsers: serde_json::Value::Null,
                    AllUsers: serde_json::Value::Null,
                    AllItems: serde_json::Value::Null,
                    PurchaseLogGlobal: serde_json::Value::Null,
                    LastPurchases: serde_json::Value::Null,
                    BillsCount: serde_json::Value::Null,
                    Bills: serde_json::Value::Null,
                    BillDetails: serde_json::Value::Null,
                    OpenFFAFreebies: serde_json::Value::Null,
                    TopPersonalDrinks: serde_json::Value::Null,
                    PurchaseLogPersonal: serde_json::Value::Null,
                    IncomingFreebies: incoming,
                    OutgoingFreebies: outgoing,
                })
            },
            a @ rustix_event_shop::BLEvents::MakeSpecialPurchase { .. } => {
                panic!("MakeSpecialPurchase supported at the moment (use CartPurchase instead!)")
            },
            a @ rustix_event_shop::BLEvents::CreateBill { .. } => {
                let _b = &mut backend.apply(&a);
                //refresh bills
                let bills = Self::query_read(&*backend, Bills(app_state.bills))?;
                Ok(RefreshedData {
                    DetailInfoForUser: serde_json::Value::Null,
                    TopUsers: serde_json::Value::Null,
                    AllUsers: serde_json::Value::Null,
                    AllItems: serde_json::Value::Null,
                    PurchaseLogGlobal: serde_json::Value::Null,
                    LastPurchases: serde_json::Value::Null,
                    BillsCount: serde_json::Value::Null,
                    Bills: bills,
                    BillDetails: serde_json::Value::Null,
                    OpenFFAFreebies: serde_json::Value::Null,
                    TopPersonalDrinks: serde_json::Value::Null,
                    PurchaseLogPersonal: serde_json::Value::Null,
                    IncomingFreebies: serde_json::Value::Null,
                    OutgoingFreebies: serde_json::Value::Null,
                })
            },
            a @ rustix_event_shop::BLEvents::FinalizeBill { .. } => {

                println!("Trying to finalize bill");
                let _b = &mut backend.apply(&a);

                println!("_b = {}", _b);

                //refresh bills
                //refresh incoming
                //refresh outgoing
                //refresh global log
                //refresh personal log
                //refresh detail info
                //refresh last purchase
                let last_log = Self::query_read(&*backend, PurchaseLogGlobal(ParametersPurchaseLogGlobal {
                    count_pars: ParametersPurchaseLogGlobalCount { millis_start: server::current_time_millis() - (1000i64 * 60 * 60 * 24), millis_end: server::current_time_millis() + 1000i64 },
                    pagination: ParametersPagination { start_inclusive: 0, end_exclusive: 5 },
                }))?;
                let bills = Self::query_read(&*backend, Bills(app_state.bills))?;

                let changed_bill_details = Self::query_read(&*backend, BillDetails(app_state.bill_detail_infos))?;

                let incoming = Self::query_read(&*backend, IncomingFreebies(app_state.incoming_freebies))?;
                let outgoing = Self::query_read(&*backend, OutgoingFreebies(app_state.outgoing_freebies))?;
                Ok(RefreshedData {
                    DetailInfoForUser: Self::query_read(&*backend, DetailInfoForUser(app_state.personal_detail_infos))?,
                    TopUsers: serde_json::Value::Null,
                    AllUsers: serde_json::Value::Null,
                    AllItems: serde_json::Value::Null,
                    PurchaseLogGlobal: Self::query_read(&*backend, PurchaseLogGlobal(app_state.global_log))?,
                    LastPurchases: last_log,
                    BillsCount: serde_json::Value::Null,
                    Bills: bills,
                    BillDetails: changed_bill_details,
                    OpenFFAFreebies: serde_json::Value::Null,
                    TopPersonalDrinks: serde_json::Value::Null,
                    PurchaseLogPersonal: Self::query_read(&*backend, PurchaseLogPersonal(app_state.personal_log))?,
                    IncomingFreebies: incoming,
                    OutgoingFreebies: outgoing,
                })
            },
            a @ rustix_event_shop::BLEvents::ExportBill { .. } => {
                let _b = &mut backend.apply(&a);
                //refresh bills
                let bills = Self::query_read(&*backend, Bills(app_state.bills))?;

                let changed_bill_details = Self::query_read(&*backend, BillDetails(app_state.bill_detail_infos))?;

                Ok(RefreshedData {
                    DetailInfoForUser: serde_json::Value::Null,
                    TopUsers: serde_json::Value::Null,
                    AllUsers: serde_json::Value::Null,
                    AllItems: serde_json::Value::Null,
                    PurchaseLogGlobal: serde_json::Value::Null,
                    LastPurchases: serde_json::Value::Null,
                    BillsCount: serde_json::Value::Null,
                    Bills: bills,
                    BillDetails: changed_bill_details,
                    OpenFFAFreebies: serde_json::Value::Null,
                    TopPersonalDrinks: serde_json::Value::Null,
                    PurchaseLogPersonal: serde_json::Value::Null,
                    IncomingFreebies: serde_json::Value::Null,
                    OutgoingFreebies: serde_json::Value::Null,
                })
            },
            a @ rustix_event_shop::BLEvents::DeleteUnfinishedBill { .. } => {
                let _b = &mut backend.apply(&a);
                //refresh bills
                let bills = Self::query_read(&*backend, Bills(app_state.bills))?;
                Ok(RefreshedData {
                    DetailInfoForUser: serde_json::Value::Null,
                    TopUsers: serde_json::Value::Null,
                    AllUsers: serde_json::Value::Null,
                    AllItems: serde_json::Value::Null,
                    PurchaseLogGlobal: serde_json::Value::Null,
                    LastPurchases: serde_json::Value::Null,
                    BillsCount: serde_json::Value::Null,
                    Bills: bills,
                    BillDetails: serde_json::Value::Null,
                    OpenFFAFreebies: serde_json::Value::Null,
                    TopPersonalDrinks: serde_json::Value::Null,
                    PurchaseLogPersonal: serde_json::Value::Null,
                    IncomingFreebies: serde_json::Value::Null,
                    OutgoingFreebies: serde_json::Value::Null,
                })
            },
            a @ rustix_event_shop::BLEvents::UpdateBill { .. } => {
                let _b = &mut backend.apply(&a);
                //refresh bills
                let bills = Self::query_read(&*backend, Bills(app_state.bills))?;

                let changed_bill_details = Self::query_read(&*backend, BillDetails(app_state.bill_detail_infos))?;

                Ok(RefreshedData {
                    DetailInfoForUser: serde_json::Value::Null,
                    TopUsers: serde_json::Value::Null,
                    AllUsers: serde_json::Value::Null,
                    AllItems: serde_json::Value::Null,
                    PurchaseLogGlobal: serde_json::Value::Null,
                    LastPurchases: serde_json::Value::Null,
                    BillsCount: serde_json::Value::Null,
                    Bills: bills,
                    BillDetails: changed_bill_details,
                    OpenFFAFreebies: serde_json::Value::Null,
                    TopPersonalDrinks: serde_json::Value::Null,
                    PurchaseLogPersonal: serde_json::Value::Null,
                    IncomingFreebies: serde_json::Value::Null,
                    OutgoingFreebies: serde_json::Value::Null,
                })
            },
                a @ rustix_event_shop::BLEvents::SetPriceForSpecial { .. } => {
                let _b = &mut backend.apply(&a);
                //refresh bills
                let bills = Self::query_read(&*backend, Bills(app_state.bills))?;

                let changed_bill_details = Self::query_read(&*backend, BillDetails(app_state.bill_detail_infos))?;

                Ok(RefreshedData {
                    DetailInfoForUser: serde_json::Value::Null,
                    TopUsers: serde_json::Value::Null,
                    AllUsers: serde_json::Value::Null,
                    AllItems: serde_json::Value::Null,
                    PurchaseLogGlobal: serde_json::Value::Null,
                    LastPurchases: serde_json::Value::Null,
                    BillsCount: serde_json::Value::Null,
                    Bills: bills,
                    BillDetails: changed_bill_details,
                    OpenFFAFreebies: serde_json::Value::Null,
                    TopPersonalDrinks: serde_json::Value::Null,
                    PurchaseLogPersonal: serde_json::Value::Null,
                    IncomingFreebies: serde_json::Value::Null,
                    OutgoingFreebies: serde_json::Value::Null,
                })
            },
            rustix_event_shop::BLEvents::MakeShoppingCartPurchase { user_id, specials, item_ids, timestamp } => {
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
                let last_log = Self::query_read(&*backend, PurchaseLogGlobal(ParametersPurchaseLogGlobal {
                    count_pars: ParametersPurchaseLogGlobalCount { millis_start: server::current_time_millis() - (1000i64 * 60 * 60 * 24), millis_end: server::current_time_millis() + 1000i64 },
                    pagination: ParametersPagination { start_inclusive: 0, end_exclusive: 5 },
                }))?;
                //refresh personal log
                let personal_log = Self::query_read(&*backend, PurchaseLogPersonal(app_state.personal_log))?;
                //do not refresh freebies (do that on-demand)

                //TODO: fix freebies

                Ok(RefreshedData {
                    DetailInfoForUser: detail_info,
                    TopUsers: top_list,
                    AllUsers: serde_json::Value::Null,
                    AllItems: serde_json::Value::Null,
                    PurchaseLogGlobal: global_log,
                    LastPurchases: last_log,
                    BillsCount: serde_json::Value::Null,
                    Bills: serde_json::Value::Null,
                    BillDetails: serde_json::Value::Null,
                    OpenFFAFreebies: serde_json::Value::Null,
                    TopPersonalDrinks: top_items,
                    PurchaseLogPersonal: personal_log,
                    IncomingFreebies: serde_json::Value::Null,
                    OutgoingFreebies: serde_json::Value::Null,
                })
            },
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
    (*back).update_user(0, "Gruin".to_string(), true, true, None);
    (*back).update_user(1, "Vall".to_string(), false, true, None);
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

    let _ = (*back).create_ffa(vec!["Liquor".to_string()], vec![0, 1, 2], 3, "I'm donating this to present how this would work in the current cervisia ui".to_string(), timestamp_counter + 1, 0);
    let _ = (*back).create_ffa(vec!["Beer".to_string()], vec![], 20, "This is the second instance which I'm donating to show how this would work in the current cervisia ui".to_string(), timestamp_counter + 10000, 1);

    (*back).create_bill(0i64, timestamp_counter + (1000i64 * 3600 * 24 * 365 * 20), UserGroup::AllUsers, "some bill comment".to_string());
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