use crate::api::models::user::{CreateUser, FindUser, UpdateUser, User};
use crate::api::repositories::base::BaseRepository;

use mongodb::error::Result;

pub struct UserRepository {
    pub base: BaseRepository<User, FindUser>,
}

#[allow(unused)]
impl UserRepository {
    pub async fn new() -> Self {
        let base = BaseRepository::<User, FindUser>::new("users");
        Self { base }
    }

    pub async fn get(&self, find_user: Option<FindUser>) -> Result<Option<Vec<User>>> {
        self.base.get(find_user).await
    }

    pub async fn create(&self, create_user: CreateUser) -> Result<Option<User>> {
        self.base.create(create_user.into()).await
    }

    pub async fn update(&self, id: String, entity: UpdateUser) -> Result<Option<User>> {
        self.base.update(id, entity.into()).await
    }

    pub async fn get_by_id(&self, id: String) -> Result<Option<User>> {
        self.base.get_by_id(id).await
    }

    pub async fn disactive_by_id(&self, id: String) -> Result<Option<User>> {
        self.base.disactive_by_id(id).await
    }

    pub async fn delete_by_id(&self, id: String) -> Result<Option<User>> {
        self.base.delete_by_id(id).await
    }
}
