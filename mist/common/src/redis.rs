use fred::prelude::*;
use serde::{de::DeserializeOwned, Serialize};
use std::marker::PhantomData;

use crate::error::Result;

pub struct TypedRedis<T> {
    prefix: &'static str,
    phantom: PhantomData<T>,
}

impl<T: Serialize + DeserializeOwned> TypedRedis<T> {
    pub const fn new(prefix: &'static str) -> Self {
        Self {
            prefix,
            phantom: PhantomData,
        }
    }

    pub fn key(&self, id: &str) -> String {
        format!("{}-{}", self.prefix, id)
    }

    pub async fn get(&self, redis: &RedisClient, id: &str) -> Result<T> {
        let fetched = redis.get::<String, _>(self.key(id)).await?;

        Ok(serde_json::from_str(&fetched)?)
    }

    pub async fn set(
        &self,
        redis: &RedisClient,
        id: &str,
        data: &T,
        expiration: Expiration,
    ) -> Result<()> {
        redis
            .set(
                self.key(id),
                serde_json::to_string(data)?,
                Some(expiration),
                None,
                false,
            )
            .await?;

        Ok(())
    }

    pub async fn del(&self, redis: &RedisClient, id: &str) -> Result<()> {
        redis.del(self.key(id)).await?;

        Ok(())
    }
}
