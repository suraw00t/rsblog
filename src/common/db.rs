use crate::api::core::config::Config;
use mongodb::{Client, Database};
use once_cell::sync::Lazy;
use std::sync::Mutex;

static MONGO_CLIENT: Lazy<Mutex<Option<Database>>> = Lazy::new(|| Mutex::new(None));

pub async fn init_db(config: &Config) {
    let client = Client::with_uri_str(&config.mongodb_uri)
        .await
        .expect("MongoDB connect failed");
    let db = client.database(&config.database_name);
    let mut mongo_guard = MONGO_CLIENT.lock().unwrap();
    *mongo_guard = Some(db);
}

pub fn get_db() -> Database {
    let mongo_guard = MONGO_CLIENT.lock().unwrap();
    mongo_guard
        .as_ref()
        .expect("MongoDB not initialized")
        .clone()
}
