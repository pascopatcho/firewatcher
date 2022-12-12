use crate::schema::twitter_credentials::{self, dsl::*};
use crate::DbPool;
use diesel::prelude::*;

type Result<T> = std::result::Result<T, diesel::result::Error>;

pub struct CredentialsManager {
    db_pool: DbPool,
}

impl CredentialsManager {
    pub fn new(db_pool: &DbPool) -> Self {
        let db_pool = db_pool.clone();

        Self { db_pool }
    }

    pub fn count(&self) -> Result<i64> {
        let mut conn = self.db_pool.get().unwrap();

        twitter_credentials.count().first(&mut conn)
    }
}
