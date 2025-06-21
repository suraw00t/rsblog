use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use utoipa::{
    openapi::schema::{Schema, SchemaFormat, SchemaType},
    ToSchema,
};

use crate::api::models::serialize_object_id;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct Token {
    #[serde(
        rename(serialize = "id", deserialize = "_id"),
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id"
    )]
    #[schema(value_type = String, example = "507f1f77bcf86cd799439011")]
    pub id: Option<ObjectId>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id"
    )]
    #[schema(value_type = String, example = "507f1f77bcf86cd799439011")]
    pub owner: Option<ObjectId>,
    pub access_token: String,
    pub refresh_token: String,
    pub access_token_expires: DateTime<Utc>,
    pub refresh_token_expires: DateTime<Utc>,
    pub is_expired: bool,
}
