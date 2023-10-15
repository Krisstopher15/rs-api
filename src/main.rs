mod config;
mod handler;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use config::Config;
use dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

struct AppState {
    db: Pool<Postgres>,
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let config = Config::new();

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => {
            println!("Connected to the database");
            pool
        }
        Err(err) => {
            println!("Failed to connect {:?}", err);
            std::process::exit(1);
        }
    };

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .configure(handler::config)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", config.port))?
    .run()
    .await
}
