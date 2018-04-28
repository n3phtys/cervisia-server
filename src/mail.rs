use configuration::*;
use lettre::{EmailAddress, EmailTransport, SimpleSendableEmail, SmtpTransport};
use lettre;
use lettre::SendableEmail;
use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::client::net::*;
use lettre::smtp::ClientSecurity;
use lettre::smtp::ConnectionReuseParameters;
use lettre::smtp::extension::ClientId;
use lettre::smtp::SUBMISSION_PORT;
use lettre_email::*;
use mime;
use native_tls::TlsConnector;
use std;
use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::Hasher;
use std::io::{Seek, Write};
use std::path::Path;
use time;
use uuid::Uuid;
use zip::result::ZipResult;
use zip::write::{FileOptions, ZipWriter};


pub fn is_too_large_for_inline(attachments: &std::collections::HashMap<String, String>) -> bool {
    let x = string_size(attachments);
    return x > 900usize;
}

pub fn string_size(attachments: &std::collections::HashMap<String, String>) -> usize {
    let mut x = 0usize;
    for (filename, filecontent) in attachments {
        x += filename.len();
        x += filecontent.len();
    }
    return x;
}

pub fn hash_code(attachments: &std::collections::HashMap<String, String>) -> u64 {
    let mut hasher = DefaultHasher::new();
    for (filename, filecontent) in attachments {
        hasher.write(filename.as_bytes());
        hasher.write(&filecontent.as_bytes());
    }
    return hasher.finish();
}

pub fn two_numbers_to_string(a: i64, b : i64) -> String {
        return format!("{:x}_{:x}", a, b);
}

pub fn save_attachments_in_zip_file(attachments: &std::collections::HashMap<String, String>, zipfilename: &str) -> String {
    //let timespec = time::get_time();
    //let mills: u64 =  timespec.sec as u64 + (timespec.nsec as f64 / 1000.0 / 1000.0 / 1000.0) as u64;
    //let hc = format!("{:x}", hash_code(attachments));

    let filename = zipfilename.to_string();
    //should no execute on (Windows + )Debug
    if !cfg!(debug_assertions) {

        let mut file = File::create(filename.to_string()).expect("Couldn't create file");
        create_zip_archive(&mut file, attachments).expect("Couldn't create archive");

        return filename;
    } else {
        let mut file = File::create(filename.to_string()).expect(&("could not create file ".to_string() + &filename));
        file.write_all(b"CERVISIA WAS COMPILED AS DEBUG! NO BILL CONTAINED HERE!").expect("Could not write file");
        return filename.to_string();
    }
}

fn create_zip_archive<T: Seek + Write>(buf: &mut T, attachments: &std::collections::HashMap<String, String>) -> ZipResult<()> {
    let mut writer = ZipWriter::new(buf);
    for (filename, filecontent) in attachments {
        writer.start_file(filename.to_string(), FileOptions::default())?;
        writer.write(filecontent.as_bytes())?;
    }
    writer.finish()?;
    Ok(())
}

pub fn send_mail(receiver_email: &str, subject: &str, body: &str, attachments: &std::collections::HashMap<String, String>, config: &ServerConfig, zipfilename: &str) -> Result<lettre::smtp::response::Response, lettre::smtp::error::Error> {


    if true {

        //implement attachments as multipart/mixed as in: https://github.com/lettre/lettre/issues/201


        //print out info in any case
        //let receivers: String = receiver_emails.clone().into_iter().map(|email| email.to_string()).fold("".to_string(), |acc,b| acc + &b);
        //warn!("Trying to send mail with to = {} :", receivers);
        //warn!("Subject: {}", subject.to_string());
        //warn!("Body: {}", body.to_string());



        match config.use_sendmail_instead_of_smtp {
            Some(true) => {
                //experimental sendmail support!
                //TODO: implement sendmail alternative
                unimplemented!()
            },
            Some(false) => {
                let my_uuid = Uuid::new_v4();
                let uuid_str = format!("{}", my_uuid);


                let mut email_builder = SimpleEmail::default();

                println!("Building email begin");


                let boundary = "XXXXboundary";
                let receiver_string = receiver_email.to_string();
                let sender_string = config.sender_email_address.to_string();
                let body_block: String = body.to_string();
                let message_id: String = format!("<{}.{}>", uuid_str.to_string(), sender_string.to_string());
                let date: String = "Thu, 18 Jan 2018 21:29:38 +0100".to_string();


                let mut attachment_blocks: String = "".to_string();

                for (filename, filecontent) in attachments {
                    //TODO: assert that filename is legit

                    attachment_blocks = attachment_blocks + &format!("--{}
Content-Type: text/plain; charset=utf-8
Content-Disposition: attachment;
        filename={}

{}


", boundary, filename, filecontent);
                }





                //let email_result = email_builder.into_email();
                //println!("Building email unwrap: {:?}", email_result);

                //let email = email_result.unwrap();

                let attachments_size = string_size(attachments);
                let email : SimpleSendableEmail = if !is_too_large_for_inline(attachments) {
                    let email_string: String = format!("Subject: {}
To: {}
From: {}
Reply-To: {}
Date: {}
MIME-Version: 1.0
Message-ID: {}
Content-Type: multipart/mixed;
        boundary={}

This is a multipart message in MIME format.

--{}
Content-Type: text/plain; charset=utf-8

{}

{}
--{}--", subject, receiver_string, sender_string, sender_string, date, message_id, boundary, boundary, body_block, attachment_blocks, boundary);


                    let email = SimpleSendableEmail::new(
                        config.sender_email_address.to_string(),
                        &vec![receiver_email.to_string()],
                        message_id,
                        email_string.to_string(),
                    ).unwrap();




                    println!("Trying to send email: {}", std::str::from_utf8(*(email.message())).unwrap());

                    let tls: ClientTlsParameters = {
                        let mut tls_builder = TlsConnector::builder().unwrap();
                        tls_builder.supported_protocols(DEFAULT_TLS_PROTOCOLS).unwrap();

                        let tls_parameters = ClientTlsParameters::new(
                            config.smtp_host_address.to_string(),
                            tls_builder.build().unwrap(),
                        );

                        tls_parameters
                    };


// Connect to a remote server on a custom port
                    let mut mailer = SmtpTransport::builder(format!("{}:{}", config.smtp_host_address, config.smtp_port), ClientSecurity::Required(tls)).unwrap()
                        // Add credentials for authentication
                        .credentials(Credentials::new(config.smpt_credentials_loginname.to_string(), config.smpt_credentials_password.to_string()))
                        // Enable SMTPUTF8 if the server supports it
                        .smtp_utf8(true)
                        // Configure expected authentication mechanism
                        .authentication_mechanism(Mechanism::Plain)
                        // Enable connection reuse
                        .connection_reuse(ConnectionReuseParameters::ReuseUnlimited).build();

                    let result_1 = mailer.send(&email);
                    println!("Sending email result: {:?}", result_1);
                    if !result_1.is_ok() {
                        println!("Error sending mail. Whole mail size was {}", attachments_size);
                    }
                    assert!(result_1.is_ok());
// Explicitly close the SMTP transaction as we enabled connection reuse
                    mailer.close();
                    return result_1;







                } else {

                    let zipfile = save_attachments_in_zip_file(attachments, zipfilename);

                    let mimetype: mime::Mime = "application/zip".parse().unwrap();

                    let email = EmailBuilder::new()
                        // Addresses can be specified by the tuple (email, alias)
                        .to((receiver_email.to_string()))
                        // ... or by an address only
                        .from(config.sender_email_address.to_string())
                        .subject(subject)
                        .text(body_block)
                        .attachment(Path::new(&zipfile), None, &mimetype).unwrap()
                        .build()
                        .unwrap();








                    println!("Trying to send email: {}", std::str::from_utf8(*(email.message())).unwrap());

                    let tls: ClientTlsParameters = {
                        let mut tls_builder = TlsConnector::builder().unwrap();
                        tls_builder.supported_protocols(DEFAULT_TLS_PROTOCOLS).unwrap();

                        let tls_parameters = ClientTlsParameters::new(
                            config.smtp_host_address.to_string(),
                            tls_builder.build().unwrap(),
                        );

                        tls_parameters
                    };


// Connect to a remote server on a custom port
                    let mut mailer = SmtpTransport::builder(format!("{}:{}", config.smtp_host_address, config.smtp_port), ClientSecurity::Required(tls)).unwrap()
                        // Add credentials for authentication
                        .credentials(Credentials::new(config.smpt_credentials_loginname.to_string(), config.smpt_credentials_password.to_string()))
                        // Enable SMTPUTF8 if the server supports it
                        .smtp_utf8(true)
                        // Configure expected authentication mechanism
                        .authentication_mechanism(Mechanism::Plain)
                        // Enable connection reuse
                        .connection_reuse(ConnectionReuseParameters::ReuseUnlimited).build();

                    println!("trying to send email: {:?}", &email);
                    let result_1 = mailer.send(&email);
                    println!("Sending email result: {:?}", result_1);
                    if !result_1.is_ok() {
                        println!("Error sending mail with zip attachment. Whole mail size would have been {} bytes", attachments_size);
                    }
// Explicitly close the SMTP transaction as we enabled connection reuse
                    mailer.close();
                    return result_1;

                };


            }
            None => return Ok(lettre::smtp::response::Response::new(
                lettre::smtp::response::Code::new(
                    lettre::smtp::response::Severity::TransientNegativeCompletion, lettre::smtp::response::Category::Unspecified4, lettre::smtp::response::Detail::Four,
                ), vec![])),
        }
    } else {
        return Ok(lettre::smtp::response::Response::new(
            lettre::smtp::response::Code::new(
                lettre::smtp::response::Severity::TransientNegativeCompletion, lettre::smtp::response::Category::Unspecified4, lettre::smtp::response::Detail::Four,
            ), vec![]));
    }
}
