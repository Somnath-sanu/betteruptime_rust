use dotenvy::dotenv;
use std::env;

pub struct Config {
    pub db_url: String,
}

impl Default for Config {
    fn default() -> Self {
        dotenv().ok();

        let db_url =
            env::var("DATABASE_URL").unwrap_or_else(|_| panic!("Please provide the database url"));

        Self { db_url } // OR return Config { db_url };
    }
}
