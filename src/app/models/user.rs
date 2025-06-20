use mongodm::mongo::{bson::doc, options::ClientOptions, Client};
use mongodm::{sync_indexes, CollectionConfig, Index, IndexOption, Indexes, Model, ToRepository};
use serde::{Deserialize, Serialize};
// field! is used to make sure at compile time that some field exists in a given structure
use mongodm::field;

pub struct UserCollConf;

impl CollectionConfig for UserCollConf {
    fn collection_name() -> &'static str {
        "user"
    }

    fn indexes() -> Indexes {
        Indexes::new()
            .with(Index::new("username").with_option(IndexOption::Unique))
            .with(Index::new("email").with_option(IndexOption::Unique))
            .with(Index::new(field!(username in User))) // field! macro can be used as well
            .with(Index::new(field!(first_name in User))) // field! macro can be used as well
            .with(Index::new(field!(last_name in User))) // field! macro can be used as well
            .with(Index::new(field!(email in User))) // field! macro can be used as well
            .with(Index::new(field!(last_login_date in User))) // field! macro can be used as well
    }
}

#[derive(Deserialize, Serialize)]
pub struct User {
    pub email: String,
    pub username: String,
    pub password: String,
    pub title_name: String,
    pub first_name: String,
    pub last_name: String,
    pub status: String,
    pub roles: String,
    pub created_date: String,
    pub updated_date: String,
    pub last_login_date: String,
}

impl Model for User {
    type CollConf = UserCollConf;
}
