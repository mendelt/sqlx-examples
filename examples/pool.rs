use sqlx::prelude::*;
use sqlx::PgPool;

const DB_URI: &str = "postgresql://example-user:password123@localhost:5432/example-database";

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = connect_db(DB_URI).await;

    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;

    Ok(())
}

async fn connect_db(uri: &str) -> PgPool {
    PgPool::connect(uri).await.unwrap()
}
