//use std::env::temp_dir;

use lettre::file::FileEmailTransport;
use lettre::{SimpleSendableEmail, EmailTransport, EmailAddress};
use configuration::*;



pub trait Mailerable {
    fn configure(config: &ServerConfig);
    fn send_mail(receiver_email: &str, header: &str, body: &str) -> bool;
}