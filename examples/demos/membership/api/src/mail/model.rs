use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub email: String,
    pub subject: String,
    pub msg: String,
}
