use deadpool_postgres::{Config as PgConfig, PoolConfig};
use std::env;

use config::ConfigError;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct SrvConfig {
    pub host: String,
    pub port: u16,
    pub secret_key: String,
    pub bcrypt_or_argon: bool,
    pub email_otp_enabled: bool,
    pub user_table_name: String,
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub smtp_tls_off: bool,
    pub user_invalid_id: i32,
    pub max_otp_attempts: i32,
}

#[derive(Deserialize, Clone)]
pub struct Config {
    pub srv_cnf: SrvConfig,
    pub pg: deadpool_postgres::Config,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        config::Config::builder()
            .add_source(config::Environment::default())
            .build()
            .unwrap()
            .try_deserialize()
    }
}

// impl Config {
//     pub fn new() -> Config {
//         let hex = env::var("SRV_CNF.SECRET_KEY").expect("SECRET_KEY not set");

//         let host = env::var("SRV_CNF.HOST").expect("HOST not set");

//         let port: u16 = if env::var("SRV_CNF.PORT").is_ok() {
//             env::var("PORT").unwrap().parse::<u16>().unwrap()
//         } else {
//             9090
//         };

//         let email_otp_enabled: bool = if env::var("SRV_CNF.ENABLE_EMAIL_OTP").is_ok() {
//             env::var("SRV_CNF.EENABLE_EMAIL_OTP")
//                 .unwrap()
//                 .parse::<bool>()
//                 .unwrap()
//         } else {
//             false
//         };

//         let bcrypt_or_argon: bool = env::var("SRV_CNF.BCRYPT_OR_ARGON").is_ok();

//         // let auth_type: AuthType = if env::var("AUTH_TYPE").is_ok() {
//         //     let t = env::var("AUTH_TYPE").unwrap();
//         //     if t.to_lowercase() == "encrypted" {
//         //         AuthType::Encrypted
//         //     } else {
//         //         AuthType::Normal
//         //     }
//         // } else {
//         //     AuthType::Normal
//         // };

//         let user_table_name: String = if env::var("SRV_CNF.USER_TABLE_NAME").is_ok() {
//             env::var("SRV_CNF.USER_TABLE_NAME").unwrap()
//         } else {
//             "users".into()
//         };

//         // let logout_url: String = if env::var("LOGOUT_URL").is_ok() {
//         //     env::var("LOGOUT_URL").unwrap()
//         // } else {
//         //     "/".into()
//         // };
//         let pg_config = PgConfig {
//             user: Some("myuser".into()),
//             password: Some("mypassword".into()),
//             dbname: Some("mydb".into()),
//             host: Some("localhost".into()),
//             port: Some(5432),
//             ..PgConfig::default()
//         };

//         Config {
//             srv_cnf: SrvConfig {
//                 host,
//                 port,
//                 secret_key: hex_to_bytes(&hex).expect("SECRET_KEY could not parse"),
//                 user_table_name,
//                 bcrypt_or_argon,
//                 email_otp_enabled,
//                 // smtp_config: SmtpConfig::new(),
//             },
//             pg: pg_config,
//             // port,
//             // auth_type,
//             // redirect_url: env::var("REDIRECT_URL").expect("REDIRECT_URL not set"),
//             // logout_url,
//             // database_url: env::var("DATABASE_URL").expect("DATABASE_URL not set"),
//             // secure_cookie: env::var("SECURE_COOKIE").is_ok(),
//             // proxy_config: ProxyConfig::new(),
//             // hit_rate: 10,
//         }
//     }
// }
