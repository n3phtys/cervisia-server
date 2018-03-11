
use rustix_bl;
use rustix_bl::rustix_event_shop;
use manager::ServableRustixImpl;
use manager::ServableRustix;
use std::env;
use std::fs::File;
use serde_json;
use std::io::prelude::*;
use rustix_bl::datastore::DatastoreQueries;
use rustix_bl::rustix_backend::WriteBackend;

#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct ImportedUser {
    pub name: String,
    pub id: String,
}

fn get_user_by_name(store: &rustix_bl::datastore::Datastore, name: &str) -> Option<rustix_bl::datastore::User> {
    let v = store.users_searchhit_ids(name);
    if v.is_empty() {
        return None;
    } else {
        let id = v.get(0).unwrap();
        return Some(store.users.get(id).unwrap().clone());
    }
}

pub fn import_users_into_store(backend: &mut rustix_bl::rustix_backend::RustixBackend, users: Vec<ImportedUser>) -> () {
    println!("Importing {} users into backend...", users.len());
    for import_user in users {
        //if nót already contained, add to list
        let first_id: Option<rustix_bl::datastore::User> = get_user_by_name(&backend.datastore, &import_user.name);
        if first_id.is_none() {
            backend.apply(&rustix_event_shop::BLEvents::CreateUser { username: import_user.name.to_string() });
            println!("Created new user {}...", import_user.name);
        }
        let existing_user : rustix_bl::datastore::User = get_user_by_name(&backend.datastore, &import_user.name).unwrap();
        //for every user in list, check if already contained (in that case, check if external_user_id is already set), update in that case
        if existing_user.external_user_id.is_none() || !existing_user.external_user_id.unwrap().eq(&import_user.id) {
            backend.apply(&rustix_event_shop::BLEvents::UpdateUser { user_id: existing_user.user_id, username: import_user.name.to_string(), is_billed: existing_user.is_billed, is_highlighted: existing_user.highlight_in_ui, external_user_id: Some(import_user.id) });
            println!("Updated user {}...", import_user.name);
        }

    }
}

pub fn load_users_json_file() -> Vec<ImportedUser> {
    let filename = "./users.json";
    let mut f_opt = File::open(filename);
    if f_opt.is_err() {
        return Vec::new();
    }
    let mut f = f_opt.unwrap();

    let mut contents = String::new();
    let read_opt = f.read_to_string(&mut contents);
    if read_opt.is_err() {
        return Vec::new();
    }

    let mut json : Vec<ImportedUser> = serde_json::from_str(&contents).unwrap_or(Vec::new());
    return json;
}