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
use server::Backend;


#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub top_items_per_user: u16,
    //default = 4
    pub server_port: u16,
    pub host: String,
    //default = 8080
    pub web_path: String,
    //default = "web/""
    pub use_persistence: bool,
    pub persistence_file_path: String,
    pub use_sendmail_instead_of_smtp: Option<bool>,
    pub sender_email_address: String,
    pub smtp_host_address: String, //also used for TLS handshake controlling
    pub smpt_credentials_loginname: String,
    pub smpt_credentials_password: String,
    pub smtp_port: u16,
}


impl Default for ServerConfig {
    fn default() -> Self {
        return ServerConfig {
            top_items_per_user: 4,
            server_port: 8081,
            host: "localhost".to_string(),
            web_path: "web/".to_string(),
            use_persistence: false,
            persistence_file_path: "./my-cervisia-lmdb.db".to_string(),
            use_sendmail_instead_of_smtp: None,
            sender_email_address: "username@hostname.org".to_string(),
            smtp_host_address: "smtp.hostname.org".to_string(),
            smpt_credentials_loginname: "username".to_string(),
            smpt_credentials_password: "s3cr3t_p@ssw0rd".to_string(),
            smtp_port: 587,
        };
    }
}


trait Loadable where Self: std::marker::Sized {
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
    let decoded: Result<ServerConfig, _> = toml::from_str(&str_incl);
    return decoded.is_ok();
}

pub fn watch_config_changes<F>(path_to_config_file: &std::path::PathBuf, function_to_execute: F) -> ()
    where F: Fn(&ServerConfig, Option<iron::Listening>, Option<Backend>) -> iron::Listening {
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


    let config_result = ServerConfig::from_path(path_to_config_file);

    if let Ok(config) = config_result {
        let s = function_to_execute(&config, old_server, None);
        old_server = Some(s);
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
                    let s = function_to_execute(&config, old_server, None);
                    old_server = Some(s);
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