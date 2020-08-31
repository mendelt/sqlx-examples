use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::prelude::*;
use sqlx::{postgres::PgDone, query, query_as, types::Json, PgPool};

const DB_URI: &str = "postgresql://example-user:password123@localhost:5432/example-database";

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = connect_db(DB_URI).await;

    let blob = Blob {
        field: "string".to_string(),
        intfield: 5,
    };

    pool.store_jsonstuff("stuff".to_string(), Json(blob))
        .await
        .unwrap();

    let stuff = pool.list_jsonstuff().await?;

    println!("{:?}", stuff);

    Ok(())
}

#[async_trait]
pub trait JsonTableRepo {
    fn get_database(&self) -> &PgPool;

    async fn store_jsonstuff(
        &self,
        name: String,
        json_stuff: Json<Blob>,
    ) -> Result<PgDone, sqlx::Error> {
        query("INSERT INTO json_table (name, json_stuff) VALUES (?, ?);")
            .bind(name)
            .bind(json_stuff)
            .execute(self.get_database())
            .await
    }

    async fn list_jsonstuff(&self) -> Result<Vec<JsonStuff>, sqlx::Error> {
        query_as::<_, JsonStuff>("SELECT id, name, json_stuff FROM json_table")
            .fetch_all(self.get_database())
            .await
    }
}

impl JsonTableRepo for PgPool {
    fn get_database(&self) -> &PgPool {
        &self
    }
}

#[derive(Debug, FromRow)]
pub struct JsonStuff {
    id: i64,
    name: String,
    json_stuff: Json<Blob>,
}

async fn connect_db(uri: &str) -> PgPool {
    PgPool::connect(uri).await.unwrap()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Blob {
    field: String,
    intfield: i64,
}
