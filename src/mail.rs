use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::SUBMISSION_PORT;
use lettre::{SimpleSendableEmail, EmailTransport, EmailAddress, SmtpTransport};
use lettre::smtp::extension::ClientId;
use lettre::smtp::ClientSecurity;
use lettre::smtp::client::net::*;
use lettre::smtp::ConnectionReuseParameters;
use lettre;
use native_tls::TlsConnector;
use uuid::Uuid;
use configuration::*;
use std;


pub fn send_mail(receiver_emails: Vec<&str>, subject: &str, body: &str, attachments: &std::collections::HashMap<&str, &str>, config: &ServerConfig) -> Result<lettre::smtp::response::Response, lettre::smtp::error::Error> {

    //TODO: implement attachments as multipart/mixed as in: https://github.com/lettre/lettre/issues/201


    //print out info in any case
    let receivers: String = receiver_emails.clone().into_iter().map(|email| email.to_string()).fold("".to_string(), |acc,b| acc + &b);
    warn!("Trying to send mail with to = {} :", receivers);
    warn!("Subject: {}", subject.to_string());
    warn!("Body: {}", body.to_string());



    match config.use_sendmail_instead_of_smtp {
        Some(true) => {
            //experimental sendmail support!
            //TODO: implement sendmail alternative
            unimplemented!()
        },
        Some(false) => {
            let my_uuid = Uuid::new_v4();
            let uuid_str = format!("{}", my_uuid);

            let email = SimpleSendableEmail::new(
                EmailAddress::new(config.sender_email_address.to_string()),
                receiver_emails.iter().map(|e| EmailAddress::new(e.to_string())).collect(),
                uuid_str,
                format!("Subject:{}\n\n{}\n", subject, body),
            );


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
            assert!(result_1.is_ok());

// Explicitly close the SMTP transaction as we enabled connection reuse
            mailer.close();
            return result_1;
        }
        None => return Ok(lettre::smtp::response::Response::new(
            lettre::smtp::response::Code::new(
                lettre::smtp::response::Severity::TransientNegativeCompletion, lettre::smtp::response::Category::Unspecified4, lettre::smtp::response::Detail(4),
            ), vec![])),
    }
}
