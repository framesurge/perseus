use config::ConfigError;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
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
    pub smtp_from_email: String,
    pub user_invalid_id: i32,
    pub max_otp_attempts: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub srv_cnf: ServerConfig,
    pub pg: deadpool_postgres::Config,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenv::dotenv().ok();

        config::Config::builder()
            .add_source(config::Environment::default().separator("__"))
            .build()?
            .try_deserialize()
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            secret_key: "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            bcrypt_or_argon: false, // Use argon2 by default
            email_otp_enabled: true,
            user_table_name: "users".to_string(),
            smtp_host: "127.0.0.1".to_string(),
            smtp_port: 1025, // Default mailpit port
            smtp_username: String::new(),
            smtp_password: String::new(),
            smtp_tls_off: true, // Mailpit doesn't use TLS by default
            smtp_from_email: "noreply@localhost".to_string(),
            user_invalid_id: -1000,
            max_otp_attempts: 10,
        }
    }
}
