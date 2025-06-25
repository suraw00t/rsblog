#![allow(unused)]
use futures::{StreamExt, TryStreamExt};
use mongodb::{
    Collection,
    bson::{Bson, doc, oid::ObjectId, to_document},
    error::{Error, Result},
};
use serde::{Serialize, de::DeserializeOwned};
use std::marker::PhantomData;

use crate::common::db::get_db;

pub struct BaseRepository<T, U>
where
    T: Send + Sync + Serialize + DeserializeOwned + Unpin + 'static,
    U: Send + Sync + Serialize + DeserializeOwned + Unpin + 'static,
{
    _marker: PhantomData<(T, U)>,
    collection_name: String,
}

impl<T, U> BaseRepository<T, U>
where
    T: Unpin + Send + Sync + Serialize + DeserializeOwned + 'static,
    U: Unpin + Send + Sync + Serialize + DeserializeOwned + 'static,
{
    pub fn new(collection_name: &str) -> Self {
        BaseRepository {
            _marker: PhantomData,
            collection_name: collection_name.to_string(),
        }
    }

    fn collection(&self) -> Collection<T> {
        let db = get_db();
        db.collection::<T>(&self.collection_name)
    }

    pub async fn get(&self, entity: Option<U>) -> Result<Option<Vec<T>>> {
        let filter = match entity {
            Some(e) => {
                let doc = to_document(&e).unwrap_or_else(|_| doc! {});
                doc
            }
            None => doc! {},
        };

        match self.collection().find(filter).await {
            Ok(cursor) => match cursor.try_collect::<Vec<T>>().await {
                Ok(items) => Ok(Some(items)),
                Err(e) => Err(e),
            },
            Err(e) => Err(Error::custom(e)),
        }
    }

    pub async fn get_by_id(&self, id: String) -> Result<Option<T>> {
        match ObjectId::parse_str(&id) {
            Ok(oid) => {
                let filter = doc! { "_id": oid };
                self.collection().find_one(filter).await
            }
            Err(e) => Err(Error::custom(e)),
        }
    }

    pub async fn create(&self, entity: T) -> Result<Option<T>> {
        match self.collection().insert_one(&entity).await {
            Ok(result) => {
                if let Some(id) = result.inserted_id.as_object_id() {
                    self.get_by_id(id.to_string()).await
                } else {
                    Ok(None)
                }
            }
            Err(e) => Err(e),
        }
    }

    pub async fn update(&self, id: String, entity: T) -> Result<Option<T>> {
        match ObjectId::parse_str(&id) {
            Ok(oid) => {
                let filter = doc! {"_id": oid};
                let update = match to_document(&entity) {
                    Ok(doc) => doc,
                    Err(e) => return Err(Error::custom(e)),
                };

                let updated_result = self
                    .collection()
                    .update_one(filter, doc! {"$set": update})
                    .await;
                match updated_result {
                    Ok(_) => self.get_by_id(id).await,
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(Error::custom(e)),
        }
    }

    pub async fn update_field_by_id(
        &self,
        id: String,
        field_name: String,
        new_value: impl Into<Bson>,
    ) -> Result<Option<T>> {
        match ObjectId::parse_str(&id) {
            Ok(oid) => {
                let filter = doc! { "_id": oid };
                let update = doc! { "$set": { field_name: new_value.into() } };

                let updated_result = self.collection().update_one(filter, update).await;
                match updated_result {
                    Ok(result) => {
                        self.get_by_id(result.upserted_id.unwrap().to_string())
                            .await
                    }
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(Error::custom(e)),
        }
    }

    pub async fn disactive_by_id(&self, id: String) -> Result<Option<T>> {
        self.update_field_by_id(id, "status".to_string(), "disactive")
            .await
    }

    pub async fn delete_by_id(&self, id: String) -> Result<Option<T>> {
        self.update_field_by_id(id, "status".to_string(), "deleted")
            .await
    }
}
