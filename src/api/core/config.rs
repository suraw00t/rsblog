use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub mongodb_uri: String,
    pub database_name: String,
}

impl Config {
    pub fn from_env() -> Self {
        Config {
            mongodb_uri: std::env::var("MONGODB_URI").expect("MONGODB_URI must be set"),
            database_name: std::env::var("DATABASE_NAME").expect("DATABASE_NAME must be set"),
        }
    }
}
