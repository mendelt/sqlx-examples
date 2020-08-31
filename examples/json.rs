use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::prelude::*;
use sqlx::{query_as, PgPool};

const DB_URI: &str = "postgresql://example-user:password123@localhost:5432/example-database";

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = connect_db(DB_URI).await;

    let stuff = pool.list_jsonstuff().await;

    println!("{:?}", stuff);

    Ok(())
}

pub trait JsonTableRepo {
    fn get_database(&self) -> &PgPool;

    async fn list_jsonstuff(&self) -> Result<Vec<JsonStuff>, sqlx::Error> {
        query_as::<_, JsonStuff>("SELECT id, name, json_stuff FROM json_table")
            .fetch_all(self.get_database)
            .await?;
    }
}

#[derive(Debug, FromRow)]
struct JsonStuff {
    id: i64,
    name: String,
    json_stuff: Value,
}

async fn connect_db(uri: &str) -> PgPool {
    PgPool::connect(uri).await.unwrap()
}
