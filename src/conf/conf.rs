use dotenv::dotenv;
use std::env;

#[derive(Clone, Debug)]
pub struct Conf {
    pub database_url: String,
    pub port: u8,
    pub host: String,
}

impl Conf {
    pub fn new() -> Self {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let host = env::var("HOST").expect("HOST is not set in .env file");
        let port = env::var("PORT")
            .expect("PORT is not set in .env file")
            .parse::<u8>()
            .unwrap();

        Conf {
            database_url,
            host,
            port,
        }
    }
}
