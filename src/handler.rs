use actix_web::{get, web, HttpResponse, Responder};

#[get("/test")]
async fn test() -> impl Responder {
    HttpResponse::Ok().body("Ok")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").service(test));
}
