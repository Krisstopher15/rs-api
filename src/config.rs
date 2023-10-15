pub struct Config {
    pub database_url: String,
    pub port: u16,
}

impl Config {
    pub fn new() -> Self {
        let database_url = std::env::var("DATABASE_URL").expect("No DATABASE_URL provided");
        let port: u16 = std::env::var("PORT")
            .expect("No PORT provided")
            .parse()
            .unwrap();

        Self { database_url, port }
    }
}
