use crate::api::core::config::Config;
use mongodb::{Client, Database};

pub async fn connect_to_mongodb(config: &Config) -> mongodb::error::Result<Database> {
    let client = Client::with_uri_str(&config.mongodb_uri).await?;
    let db = client.database(&config.database_name);
    Ok(db)
}
