use mongodb::{
    bson::{doc, oid::ObjectId, Bson},
    error::Error as MongoError,
    error::Result,
    Collection,
};
use serde::{de::DeserializeOwned, Serialize};
use std::marker::PhantomData;

pub struct BaseRepository<T>
where
    T: Send + Sync + Serialize + DeserializeOwned + Unpin + 'static,
{
    pub collection: Collection<T>,
    _marker: PhantomData<T>,
}

#[allow(unused)]
impl<T> BaseRepository<T>
where
    T: Unpin + Send + Sync + Serialize + DeserializeOwned + 'static,
{
    pub fn new(collection: Collection<T>) -> Self {
        BaseRepository {
            collection,
            _marker: PhantomData,
        }
    }

    // pub async fn get_many(&self) -> Result<Option<T>> {
    //     Ok(MongoError());
    //     // Ok(())
    // }

    pub async fn get_by_id(&self, id: String) -> Result<Option<T>> {
        match ObjectId::parse_str(&id) {
            Ok(oid) => {
                let filter = doc! { "_id": oid };
                self.collection.find_one(filter).await
            }
            Err(_) => Ok(None), // or return Err(MongoError::...) if you want
        }
    }

    pub async fn create(&self, entity: T) -> Result<Option<T>> {
        match self.collection.insert_one(&entity).await {
            Ok(result) => {
                if let Some(id) = result.inserted_id.as_object_id() {
                    self.get_by_id(id.to_string()).await
                } else {
                    Ok(None)
                }
            }
            Err(_) => Ok(None),
        }
    }

    pub async fn update_field_by_id(
        &self,
        id: String,
        field_name: &str,
        new_value: impl Into<Bson>,
    ) -> Result<Option<T>> {
        match ObjectId::parse_str(&id) {
            Ok(oid) => {
                let filter = doc! { "_id": oid };
                let update = doc! { "$set": { field_name: new_value.into() } };
                let updated_result = self.collection.update_one(filter, update).await;

                match updated_result {
                    Ok(_) => self.get_by_id(id).await,
                    Err(_) => Ok(None),
                }
            }
            Err(_) => Ok(None),
        }
    }

    // pub async fn disactive_by_id(&self, id: String) -> Result<Option<T>> {
    pub async fn disactive_by_id(&self, id: String) -> Result<()> {
        Ok(())
    }

    // pub async fn delete_by_id(&self, id: String) -> Result<Option<T>> {
    pub async fn delete_by_id(&self, id: String) -> Result<()> {
        Ok(())
    }
}

// pub trait BaseRepository<T> {
//     fn get_by_id(&self, id: u32) -> Option<T>;
//     fn create(&mut self, entity: T);
// }

// impl BaseRepository {
//     fn get_list(&self) {}
//     fn get_by_id(&self, id: String) {}
//     fn create(&self) {}
//     fn update(&self) {}
//     fn delete_by_id(&self, id: String) {}
//     fn disactive_by_id(&self, id: String) {}
// }
