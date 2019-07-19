use rustix_bl;
use rustix_bl::datastore::DatastoreQueries;
use rustix_bl::rustix_backend::WriteBackend;
use rustix_bl::rustix_event_shop;
use serde_json;
use std::fs::File;
use std::io::prelude::*;

#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct ImportedUser {
    pub name: String,
    pub id: String,
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct ImportedItem {
    pub name: String,
    pub category: String,
    pub price: u32,
}

fn get_user_by_name(
    store: &rustix_bl::datastore::Datastore,
    name: &str,
) -> Option<rustix_bl::datastore::User> {
    let v = store.users_searchhit_ids(name);
    if v.is_empty() {
        return None;
    } else {
        let id = v.get(0).unwrap();
        return Some(store.users.get(id).unwrap().clone());
    }
}

fn get_item_by_name(
    store: &rustix_bl::datastore::Datastore,
    name: &str,
) -> Option<rustix_bl::datastore::Item> {
    let v = store.items_searchhit_ids(name);
    if v.is_empty() {
        return None;
    } else {
        let id = v.get(0).unwrap();
        return Some(store.items.get(id).unwrap().clone());
    }
}

pub fn import_users_into_store(
    backend: &mut rustix_bl::rustix_backend::RustixBackend,
    users: Vec<ImportedUser>,
) -> () {
    println!("Importing {} users into backend...", users.len());
    for import_user in users {
        //if nót already contained, add to list
        let first_id: Option<rustix_bl::datastore::User> =
            get_user_by_name(&backend.datastore, &import_user.name);
        if first_id.is_none() {
            backend.apply(&rustix_event_shop::BLEvents::CreateUser {
                username: import_user.name.to_string(),
            });
            println!("Created new user {}...", import_user.name);
        }
        let opt = get_user_by_name(&backend.datastore, &import_user.name);
        if opt.is_some() {
            let existing_user: rustix_bl::datastore::User =
                opt.unwrap();
            //for every user in list, check if already contained (in that case, check if external_user_id is already set), update in that case
            if existing_user.external_user_id.is_none()
                || !existing_user.external_user_id.unwrap().eq(&import_user.id)
            {
                backend.apply(&rustix_event_shop::BLEvents::UpdateUser {
                    user_id: existing_user.user_id,
                    username: import_user.name.to_string(),
                    is_billed: existing_user.is_billed,
                    is_sepa: existing_user.is_sepa,
                    is_highlighted: existing_user.highlight_in_ui,
                    external_user_id: Some(import_user.id),
                });
                println!("Updated user {}...", import_user.name);
            }
        }
    }
}

pub fn import_items_into_store(
    backend: &mut rustix_bl::rustix_backend::RustixBackend,
    items: Vec<ImportedItem>,
) -> () {
    println!("Importing {} items into backend...", items.len());
    for import_item in items {
        //if nót already contained, add to list
        let first_id: Option<rustix_bl::datastore::Item> =
            get_item_by_name(&backend.datastore, &import_item.name);
        let cat: Option<String> = if import_item.category.trim().len() == 0 {
            None
        } else {
            Some(import_item.category.trim().to_string())
        };

        if first_id.is_none() {
            backend.apply(&rustix_event_shop::BLEvents::CreateItem {
                itemname: import_item.name.to_string(),
                price_cents: import_item.price,
                category: cat,
            });
            println!("Created new item {}...", import_item.name);
        } else {
            backend.apply(&rustix_event_shop::BLEvents::UpdateItem {
                item_id: first_id.unwrap().item_id,
                itemname: import_item.name.to_string(),
                price_cents: import_item.price,
                category: cat,
            });
            println!("Updated item {}...", import_item.name);
        }
    }
}

pub fn load_users_json_file() -> Vec<ImportedUser> {
    let filename = "./users.json";
    let f_opt = File::open(filename);
    if f_opt.is_err() {
        return Vec::new();
    }
    let mut f = f_opt.unwrap();

    let mut contents = String::new();
    let read_opt = f.read_to_string(&mut contents);
    if read_opt.is_err() {
        return Vec::new();
    }

    let json: Vec<ImportedUser> = serde_json::from_str(&contents).unwrap_or(Vec::new());
    return json;
}

pub fn load_items_json_file() -> Vec<ImportedItem> {
    let filename = "./items.json";
    let f_opt = File::open(filename);
    if f_opt.is_err() {
        return Vec::new();
    }
    let mut f = f_opt.unwrap();

    let mut contents = String::new();
    let read_opt = f.read_to_string(&mut contents);
    if read_opt.is_err() {
        return Vec::new();
    }

    let json: Vec<ImportedItem> = serde_json::from_str(&contents).unwrap_or(Vec::new());
    return json;
}
