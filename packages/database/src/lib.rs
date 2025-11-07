use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub type DbPool = Pool<Postgres>;

#[derive(Clone)]
pub struct Database {
    pub pool: DbPool,
}

impl Database {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    // Example method - we'll add more later
    pub async fn health_check(&self) -> Result<(), sqlx::Error> {
        sqlx::query("SELECT 1").execute(&self.pool).await?;
        Ok(())
    }
}
EOF