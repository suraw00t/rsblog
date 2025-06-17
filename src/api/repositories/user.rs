use crate::api::models::user::{CreateUser, User};
use crate::api::repositories::base::BaseRepository;

use mongodb::{error::Result, Database};

pub struct UserRepository {
    pub base: BaseRepository<User>,
}

#[allow(unused)]
impl UserRepository {
    pub async fn new(db: &Database) -> Self {
        let collection = db.collection::<User>("users");
        let base = BaseRepository::new(collection);
        Self { base }
    }

    // Optional: expose base methods directly
    pub async fn create(&self, create_user: CreateUser) -> Result<Option<User>> {
        self.base.create(create_user.into()).await
    }

    pub async fn get_by_id(&self, id: String) -> Result<Option<User>> {
        self.base.get_by_id(id).await
    }
}
