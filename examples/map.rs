use sqlx::prelude::*;
use sqlx::{query_as, PgPool};

const DB_URI: &str = "postgresql://example-user:password123@localhost:5432/example-database";

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = connect_db(DB_URI).await;

    let stuff = query_as::<_, Stuff>( 
        "SELECT id, name, value FROM example_table")
        .fetch_all(&pool)
        .await?;

    println!("{:?}", stuff);

    Ok(())
}

#[derive(Debug, FromRow)]
struct Stuff {
    id: i64,
    name: String,
    value: String,
}

async fn connect_db(uri: &str) -> PgPool {
    PgPool::connect(uri).await.unwrap()
}
