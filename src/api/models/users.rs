use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::api::models::serialize_object_id;

#[derive(Serialize, Deserialize, Debug, ToSchema, Clone)]
pub struct BaseUser {
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema, Clone)]
pub struct CreateUser(pub BaseUser);

#[derive(Serialize, Deserialize, Debug, ToSchema, Clone)]
pub struct UpdateUser(pub BaseUser);

#[derive(Serialize, Deserialize, Debug, ToSchema, Clone, IntoParams)]
pub struct FindUser {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema, Clone)]
pub struct User {
    #[serde(
        rename(serialize = "id", deserialize = "_id"),
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id"
    )]
    #[schema(value_type = String, example = "507f1f77bcf86cd799439011")]
    pub id: Option<ObjectId>,
    #[serde(flatten)]
    pub base: BaseUser,
}

impl From<CreateUser> for User {
    fn from(create_user: CreateUser) -> Self {
        User {
            id: None,
            base: create_user.0,
        }
    }
}

impl From<UpdateUser> for User {
    fn from(update_user: UpdateUser) -> Self {
        User {
            id: None,
            base: update_user.0,
        }
    }
}
