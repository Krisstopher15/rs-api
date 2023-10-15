#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub port: u16,
    pub secret_password: String,
}

impl Config {
    pub fn new() -> Self {
        let database_url = std::env::var("DATABASE_URL").expect("No DATABASE_URL provided");
        let port: u16 = std::env::var("PORT")
            .expect("No PORT provided")
            .parse()
            .unwrap();
        let secret_password =
            std::env::var("PASSWORD_SECRET").expect("No PASSWORD_SECRET provided");

        Self {
            database_url,
            port,
            secret_password,
        }
    }
}
