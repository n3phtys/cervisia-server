use std;
use std::env;
use std::fs::File;
use std::io;
use std::io::Read;
use toml;


#[derive(Debug, Deserialize, Clone)]
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
    pub use_mock_data: bool,
    pub admin_password: String,
}


impl ServerConfig {
    pub fn inline_default_config() -> ServerConfig {
        return ServerConfig {
            top_items_per_user: 4,
            server_port: 8080,
            host: "localhost".to_string(),
            web_path: "dist/".to_string(),
            use_persistence: false,
            persistence_file_path: "./my-cervisia-lmdb.db".to_string(),
            use_sendmail_instead_of_smtp: None,
            sender_email_address: "username@hostname.org".to_string(),
            smtp_host_address: "smtp.hostname.org".to_string(),
            smpt_credentials_loginname: "username".to_string(),
            smpt_credentials_password: "s3cr3t_p@ssw0rd".to_string(),
            smtp_port: 587,
            use_mock_data: true,
            admin_password: "".to_string(),
        };
    }

    pub fn from_env() -> ServerConfig {
        return ServerConfig {
            top_items_per_user: get_env_u16("CERVISIA_NUMBER_OF_TOP_ITEMS", 4),
            server_port: get_env_u16("CERVISIA_SERVER_PORT", 8080),
            host: env::var("CERVISIA_SERVER_HOST").unwrap_or("localhost".to_string()),
            web_path: env::var("CERVISIA_WEB_PATH").unwrap_or("dist/".to_string()),
            use_persistence: ! env::var("CERVISIA_PERSISTENCE_PATH").is_err(),
            persistence_file_path: env::var("CERVISIA_PERSISTENCE_PATH").unwrap_or("./my-cervisia-lmdb.db".to_string()),
            use_sendmail_instead_of_smtp: get_env_bool("CERVISIA_SMTP_USE_SENDMAIL", None),
            sender_email_address: env::var("CERVISIA_SMTP_SENDER").unwrap_or("username@hostname.org".to_string()),
            smtp_host_address: env::var("CERVISIA_SMTP_HOST").unwrap_or("smtp.hostname.org".to_string()),
            smpt_credentials_loginname: env::var("CERVISIA_SMTP_USERNAME").unwrap_or("username".to_string()),
            smpt_credentials_password: env::var("CERVISIA_SMTP_PASSWORD").unwrap_or("s3cr3t_p@ssw0rd".to_string()),
            smtp_port: get_env_u16("CERVISIA_SMTP_PORT", 587),
            use_mock_data: get_env_bool("CERVISIA_USE_MOCK_DATA", Some(true)).unwrap_or(true),
            admin_password: env::var("CERVISIA_ADMIN_PASSWORD").unwrap_or("".to_string()),
        };
    }
}

fn get_env_u16(key: &str, def: u16) -> u16 {
    match env::var(key) {
        Ok(s) => {
            let x = s.parse::<u16>();
            return match x {
                Ok(v) => v,
                Err(_) => def,
            };
        },
        Err(_) => {
            return def;
        },
    }
}

fn get_env_bool(key: &str, def : Option<bool>) -> Option<bool> {
    match env::var(key) {
        Ok(s) => {
            let x = s.parse::<bool>();
            return match x {
                Ok(true) => Some(true),
                Ok(false) => Some(false),
                Err(_) => def,
            };
        },
        Err(_) => {
            return def;
        },
    }
}



impl Default for ServerConfig {
    fn default() -> Self {
        return ServerConfig {
            top_items_per_user: 4,
            server_port: 8081,
            host: "localhost".to_string(),
            web_path: "dist/".to_string(),
            use_persistence: false,
            persistence_file_path: "./my-cervisia-lmdb.db".to_string(),
            use_sendmail_instead_of_smtp: None,
            sender_email_address: "username@hostname.org".to_string(),
            smtp_host_address: "smtp.hostname.org".to_string(),
            smpt_credentials_loginname: "username".to_string(),
            smpt_credentials_password: "s3cr3t_p@ssw0rd".to_string(),
            smtp_port: 587,
            use_mock_data: true,
            admin_password: "".to_string(),
        };
    }
}


trait Loadable where Self: std::marker::Sized {
    fn from_path(path: &std::path::PathBuf) -> Result<Self, io::Error>;
}

impl Loadable for ServerConfig {
    fn from_path(path: &std::path::PathBuf) -> Result<Self, io::Error> {
        let file_raw = File::open(path);

        if file_raw.is_err() {
            return Ok(ServerConfig::inline_default_config())
        }

        let mut file = file_raw?;

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
        //let path3 = path2.clone();
        //let mut k = File::create(path3).unwrap();
        //let str_incl = include_str!("SettingsDefault.toml");
        //k.write_all(
        //    str_incl.as_bytes()).unwrap();
    }
    return path2;
}
