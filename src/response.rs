use serde::Serialize;

#[derive(Serialize)]
pub struct RegisterUserResponse {
    pub name: String,
    pub lastname: String,
    pub username: String,
    pub email: String,
}
