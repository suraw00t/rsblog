use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize, Serializer};
use utoipa::ToSchema;

#[derive(Deserialize, Debug, ToSchema)]
pub struct User {
    #[serde(
        rename(serialize = "id", deserialize = "_id"),
        skip_serializing_if = "Option::is_none"
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

// Implement custom Serialize for User
impl Serialize for User {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("User", 3)?;
        if let Some(id) = &self.id {
            state.serialize_field("id", &id.to_hex())?;
        } else {
            state.serialize_field("id", &Option::<String>::None)?;
        }
        state.serialize_field("name", &self.name)?;
        state.serialize_field("email", &self.email)?;
        state.end()
    }
}
