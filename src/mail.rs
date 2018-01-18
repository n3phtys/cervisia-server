use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::SUBMISSION_PORT;
use lettre::{SimpleSendableEmail, EmailTransport, EmailAddress, SmtpTransport};
use lettre::smtp::extension::ClientId;
use lettre::smtp::ClientSecurity;
use lettre::smtp::client::net::*;
use lettre::smtp::ConnectionReuseParameters;
use lettre;
use time;
use native_tls::TlsConnector;
use uuid::Uuid;
use configuration::*;
use lettre_email::*;
use lettre::SendableEmail;
use std::path::Path;
use std;


pub fn send_mail(receiver_email: &str, subject: &str, body: &str, attachments: &std::collections::HashMap<&str, &str>, config: &ServerConfig) -> Result<lettre::smtp::response::Response, lettre::smtp::error::Error> {

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

                /*email_builder = email_builder
                    .from(config.sender_email_address.to_string())
                    .reply_to(config.sender_email_address.to_string())
                    .mi
                    .text(body)
                    .date(time::now().clone())
                    .subject(subject);

                for receiver in &receiver_emails {
                    email_builder = email_builder.to(*receiver);
                }*/

                /*for (filename, filecontent) in attachments {
                    let attachstring: String = format!("Content-Disposition: attachment; filename=\"{}\"
Content-Type: text/plain\n\n{}", filename, filecontent);
                    println!("Attaching attachment:\n{}", attachstring);
                    let attachref: &str = &attachstring;

                    email_builder = email_builder.attachment("Cargo.toml");
                }*/

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


                let email_string : String = format!("Subject: {}
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
--{}--", subject, receiver_string, sender_string, sender_string, date,message_id, boundary, boundary, body_block, attachment_blocks, boundary );


                //let email_result = email_builder.into_email();
                //println!("Building email unwrap: {:?}", email_result);

                //let email = email_result.unwrap();

                let email = SimpleSendableEmail::new(
                    EmailAddress::new(config.sender_email_address.to_string()),
                    vec![receiver_email.to_string()].iter().map(|e| EmailAddress::new(e.to_string())).collect(),
                    message_id,
                    email_string,
                );


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
