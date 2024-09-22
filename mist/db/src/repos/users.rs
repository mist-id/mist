use async_trait::async_trait;
use common::Result;
use sqlx::{query_file_as, PgPool};

use crate::models::user::{CreateUser, User, UserId};

#[async_trait]
#[mockall::automock]
pub trait UserRepo: Send + Sync {
    async fn create(&self, data: &CreateUser) -> Result<User>;
    async fn get(&self, id: &UserId) -> Result<User>;
}

pub struct PgUserRepo {
    pool: PgPool,
}

impl PgUserRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepo for PgUserRepo {
    async fn create(&self, data: &CreateUser) -> Result<User> {
        let user = query_file_as!(
            User,
            "sql/users/create.sql",
            data.service_id,
            data.id.as_ref()
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    async fn get(&self, id: &UserId) -> Result<User> {
        let user = query_file_as!(User, "sql/users/get.sql", id.as_ref())
            .fetch_one(&self.pool)
            .await?;

        Ok(user)
    }
}
