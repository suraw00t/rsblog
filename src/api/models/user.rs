use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize, Serializer};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct User {
    #[serde(
        rename(serialize = "id", deserialize = "_id"),
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id"
    )]
    #[schema(value_type = String, example = "507f1f77bcf86cd799439011")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub email: String,
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

fn serialize_object_id<S>(id: &Option<ObjectId>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match id {
        Some(object_id) => serializer.serialize_str(&object_id.to_hex()),
        None => serializer.serialize_none(),
    }
}
