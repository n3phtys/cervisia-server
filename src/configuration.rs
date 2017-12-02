use std;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use config::*;
use config;
use notify::{RecommendedWatcher, DebouncedEvent, Watcher, RecursiveMode};
use std::sync::mpsc::channel;
use std::time::Duration;
use rustix_bl;
use iron;
use toml;
use std::io;
use std::sync::{Arc, RwLock};
use std::thread;


#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub use_send_mail: bool,
    pub email_server: String,
    pub email_username: String,
    pub email_password: String,
    pub top_items_per_user: u16,
    //default = 4
    pub server_port: u16,
    //default = 8080
}


impl Default for ServerConfig {
    fn default() -> Self {
        return ServerConfig {
            use_send_mail: false,
            email_server: String::new(),
            email_username: String::new(),
            email_password: String::new(),
            top_items_per_user: 4,
            server_port: 8081,
        };
    }
}


trait Loadable  where Self: std::marker::Sized{
    fn from_path(path: &std::path::PathBuf) -> Result<Self, io::Error>;
}

impl Loadable for ServerConfig {
    fn from_path(path: &std::path::PathBuf) -> Result<Self, io::Error> {
        let mut file = File::open(path)?;
        let mut s = String::new();
        file.read_to_string(&mut s)?;
        let decoded: ServerConfig = toml::from_str(&s).unwrap();
        return Ok(decoded);
    }
}


pub fn path_to_config_file_and_mkdirs() -> std::path::PathBuf {
    let mut path = std::env::home_dir().unwrap();
    path.push(".cervisia-server");

    println!("A2");

    {
        let path = path.clone();
        let _ = std::fs::create_dir_all(path);
    }
    {
        path.push("Settings");
        path.set_extension("toml");
    }

    let path2 = path.clone();

    let f_opt = File::open(path);

    if f_opt.is_ok() {
        println!("Config file found in {:?}", path2);
    } else {
        let path3 = path2.clone();
        let mut k = File::create(path3).unwrap();
        let str_incl = include_str!("SettingsDefault.toml");
        k.write_all(
            str_incl.as_bytes()).unwrap();
    }
    return path2;
}


fn assert_default_settings_parse() -> bool {
    let str_incl = include_str!("SettingsDefault.toml");
    let decoded: Result<ServerConfig,_> = toml::from_str(&str_incl);
    return decoded.is_ok();
}

pub fn watch_config_changes<F>(path_to_config_file: &std::path::PathBuf, function_to_execute: F) -> ()
    where F: Fn(&ServerConfig, Option<Arc<RwLock<rustix_bl::rustix_backend::RustixBackend<rustix_bl::persistencer::TransientPersister>>>>, Option<iron::Listening>) -> (Arc<RwLock<rustix_bl::rustix_backend::RustixBackend<rustix_bl::persistencer::TransientPersister>>>, iron::Listening) {


    println!("Here");
    //assert that default is right
    if !assert_default_settings_parse() {
        panic!("SettingsDefault.toml is not parsing!");
    }



    println!("There");
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2)).unwrap();


    debug!("Spawned watcher");


    println!("Also here");

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.

    let mut old_server: Option<iron::Listening> = None;
    let mut old_backend: Option<Arc<RwLock<rustix_bl::rustix_backend::RustixBackend<rustix_bl::persistencer::TransientPersister>>>> = (None);


    let config_result = ServerConfig::from_path(path_to_config_file);

    if let Ok(config) = config_result {
        let (b, s) = function_to_execute(&config, old_backend, old_server);
        old_server = Some(s);
        old_backend = Some(b);
    } else {
        println!("Error during Config parsing: {:?}", config_result);
    }


    watcher
        .watch(path_to_config_file, RecursiveMode::NonRecursive)
        .unwrap();

    // This is a simple loop, but you may want to use more complex logic here,
    // for example to handle I/O.
    loop {

        debug!("Loop");

        match rx.recv() {
            Ok(DebouncedEvent::Write(_)) => {
                println!(" * Settings.toml changed; refreshing configuration ...");

                let config_result = ServerConfig::from_path(path_to_config_file);

                if let Ok(config) = config_result {
                    let (b, s) = function_to_execute(&config, old_backend, old_server);
                    old_server = Some(s);
                    old_backend = Some(b);
                } else {
                    println!("Error during Config parsing: {:?}", config_result);
                }
            }

            Err(e) => println!("watch error: {:?}", e),

            _ => {
                // Ignore event
            }
        }
    }
}