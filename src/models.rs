use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UserRegister {
    pub name: String,
    pub lastname: String,
    pub username: String,
    pub email: String,
    pub password: String,
}
