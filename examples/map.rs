use sqlx::prelude::*;
use sqlx::{query_as, PgPool};
use serde_json::Value;

const DB_URI: &str = "postgresql://example-user:password123@localhost:5432/example-database";

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = connect_db(DB_URI).await;

    let stuff = query_as!(Stuff, 
        "SELECT id, name, json_stuff FROM example_table")
        .fetch_all(&pool)
        .await?;

    Ok(())
}

struct Stuff {
    id: i64,
    name: String,
    json_stuff: Value,
}

async fn connect_db(uri: &str) -> PgPool {
    PgPool::connect(uri).await.unwrap()
}
