use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use utoipa::{IntoParams, ToResponse, ToSchema};

#[derive(
    Serialize, Debug, ToSchema, ToResponse, IntoParams, Deserialize, PostgresMapper, Default,
)]
#[schema(title = "Register")]
#[schema(example = json!({"class": "form inline"}))]
#[response(description = "Register your Account")]
#[pg_mapper(table = "users")]
pub struct CreateUser {
    #[schema(example = json!({"widget": "email", "class": "email inline"}))]
    pub email: String,
    #[schema(example = json!({"widget": "password", "class": "email-form px-2"}))]
    pub hashed_password: String,
}

#[derive(Serialize, Debug, Deserialize, PostgresMapper, Default)]
#[pg_mapper(table = "users")]
pub struct UserPw {
    pub id: i32,
    pub hashed_password: String,
}

#[derive(Serialize, Debug, Deserialize, PostgresMapper, Default)]
#[pg_mapper(table = "users")]
pub struct UserValidateHash {
    pub id: i32,
    pub reset_password_validator_hash: String,
}

#[derive(Serialize, Debug, Deserialize, PostgresMapper, Default)]
#[pg_mapper(table = "sessions")]
pub struct UserSession {
    pub id: i32,
    pub user_id: i32,
    pub session_verifier: String,
    pub otp_code_confirmed: bool,
    pub otp_code_encrypted: String,
    pub otp_code_attempts: i32,
    pub otp_code_sent: bool,
}

#[derive(Serialize, Debug, Deserialize, Default)]
pub struct SessionAdd {
    pub user_id: i32,
    pub session_verifier: String,
    pub otp_code_encr: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub session_id: i32,
    pub session_verifier: String,
    pub master_key_hash: Option<String>,
}

// #[derive(Serialize)]
// pub struct UserUISchema {
//     pub email: UISchemaField,
//     pub hashed_password: UISchemaField,
// }

// #[derive(Serialize)]
// pub struct UISchemaField {
//     #[serde(rename = "ui:widget")]
//     pub widget: String,
//     #[serde(skip_serializing_if = "Option::is_none", rename = "ui:title")]
//     pub title: Option<String>,
//     #[serde(skip_serializing_if = "Option::is_none", rename = "ui:description")]
//     pub description: Option<String>,
// }
/// Todo endpoint error responses
// #[derive(Serialize, Deserialize, Clone, ToSchema)]
// pub enum ErrorResponse {
//     /// When Todo is not found by search term.
//     NotFound(String),
//     /// When there is a conflict storing a new todo.
//     Conflict(String),
//     /// When todo endpoint was called without correct credentials
//     Unauthorized(String),
// }

#[derive(Serialize, Deserialize, Default)]
pub struct Reset {
    pub password: String,
    pub reset_password_selector: String,
    pub reset_password_validator: String,
}

#[derive(Deserialize)]
pub struct Params {
    pub reset_password_selector: String,
    pub reset_password_validator: String,
}

#[derive(Serialize, Deserialize, PostgresMapper, Default)]
#[pg_mapper(table = "users")]
pub struct UserMail {
    pub email: String,
}

#[derive(Serialize, Deserialize, PostgresMapper, Default)]
#[pg_mapper(table = "users")]
pub struct CreatedUser {
    pub id: i32,
}

#[derive(Serialize, Deserialize, PostgresMapper, Debug, Default)]
#[pg_mapper(table = "sessions")]
pub struct CreatedSession {
    pub id: i32,
}
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Otp {
    pub code: String,
    pub session_id: i32,
    pub session_verifier: String,
}
