use actix_web::{get, post, web, HttpResponse, Responder};
use argonautica::Hasher;

use crate::{models::UserRegister, response::RegisterUserResponse, AppState};

#[get("/test")]
async fn test() -> impl Responder {
    HttpResponse::Ok().body("Ok")
}

#[post("/auth/register")]
async fn register_handler(
    data: web::Data<AppState>,
    body: web::Json<UserRegister>,
) -> impl Responder {
    let secret = &data.config.secret_password;

    let mut hasher = Hasher::default();
    let hash = hasher
        .with_password(&body.password)
        .with_secret_key(secret)
        .hash()
        .unwrap();

    let response = RegisterUserResponse {
        name: body.name.to_owned(),
        lastname: body.lastname.to_owned(),
        username: body.username.to_owned(),
        email: body.email.to_owned(),
    };

    HttpResponse::Accepted().json(serde_json::json!({"response": response, "password_hash": hash}))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").service(test).service(register_handler));
}
