use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::api::models::serialize_object_id;

#[derive(Serialize, Deserialize, Debug, ToSchema, Clone)]
pub struct BaseUser {
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct CreateUser(pub BaseUser);

#[derive(Serialize, Deserialize, Debug, ToSchema)]
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

impl User {
    pub fn with_id(mut self, id: ObjectId) -> Self {
        self.id = Some(id);
        self
    }

    pub fn id(&self) -> Option<String> {
        self.id.map(|id| id.to_hex())
    }
}
