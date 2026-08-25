use crate::configs;

use lettre::{Message, SmtpTransport, Transport};

pub fn send_email(email: Message) {
    let config = configs::Config::from_env().unwrap();

    // let creds = Credentials::new(, smtp_config.password.clone());

    let sender = if config.srv_cnf.smtp_tls_off {
        SmtpTransport::builder_dangerous(config.srv_cnf.smtp_host)
            .port(config.srv_cnf.smtp_port)
            // .credentials(creds)
            .build()
    } else {
        SmtpTransport::relay(&config.srv_cnf.smtp_host)
            .unwrap()
            .port(config.srv_cnf.port)
            // .credentials(creds)
            .build()
    };

    // Send the email
    match sender.send(&email) {
        Ok(_) => println!("Email sent"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
    // if let Some(smtp_config) = &config.smtp_config {
    // }
}

// // use super::model::*;
// use html2text::from_read;
// use mail_builder::MessageBuilder;
// use mail_send::SmtpClientBuilder;

// use super::Message;

// pub async fn send_email(email: Message) {
//     let config = configs::Config::from_env().unwrap();
//     // let creds = Credentials::new(config.srv_cnf.smtp_username, config.srv_cnf.smtp_password);
//     let default_email = "me@me.com";

//     let text_mail = from_read(email.msg.as_bytes(), 80);

//     let message = MessageBuilder::new()
//         .from(default_email)
//         .to(email.email)
//         .subject(email.subject)
//         .html_body(email.msg)
//         .text_body(text_mail);

//     let mut sender = SmtpClientBuilder::new(config.srv_cnf.smtp_host, config.srv_cnf.port)
//         .implicit_tls(false)
//         // .credentials(("john", "p4ssw0rd"))
//         .connect()
//         .await
//         .unwrap();
//     // .send(message)
//     // .await
//     // .unwrap();

//     // Send the email
//     match &sender.send(message).await {
//         Ok(_) => println!("Email sent successfully!"),
//         Err(e) => panic!("Could not send email: {:?}", e),
//     }

//     // if let Some(smtp_config) = &config.src_cnf {
//     // }
// }
