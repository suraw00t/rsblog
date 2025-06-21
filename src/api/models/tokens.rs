use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Token {
    #[serde(rename(serialize = "id", deserialize = "_id"))]
    pub id: Option<ObjectId>,
    pub owner: Option<ObjectId>,
    pub access_token: String,
    pub refresh_token: String,
    pub access_token_expires: DateTime,
    pub refresh_token_expires: DateTime,
    pub is_expired: bool,
}
