use crate::common::config::Config;
use mongodb::{options::ClientOptions, Client, Database};
use once_cell::sync::Lazy;
use std::sync::Mutex;

static MONGO_CLIENT: Lazy<Mutex<Option<Database>>> = Lazy::new(|| Mutex::new(None));

pub async fn init_db() {
    let client_options = ClientOptions::parse(Config::get_database_uri())
        .await
        .expect("Failed to parse MongoDB URI");
    let client = Client::with_options(client_options).expect("MongoDB connect failed");
    let db = client
        .default_database()
        .expect("No default database specified in URI");
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
