use serde::{Serialize, Deserialize};
use sqlx::PgPool;

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone)]
pub struct Todo {
    pub id: i64,
    pub text: String,
    pub done: bool
}

impl Todo {
    pub async fn get_all(pool: &PgPool) -> Vec<Todo> {
        sqlx::query_as::<_, Todo>("SELECT * FROM todos ORDER BY id")
            .fetch_all(pool)
            .await.expect("Failed to load Todos from DB")
    }

    pub async fn get_by_id(pool: &PgPool, id: i64) -> Option<Todo> {
        sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await.ok()
    }

    pub async fn create(&self, pool: &PgPool) {
        sqlx::query("INSERT INTO todos (text) VALUES ($1)")
            .bind(&self.text)
            .execute(pool)
            .await.expect("Couldn't create Todo in DB");
    }

    pub async fn update(&self, pool: &PgPool) {
        sqlx::query("UPDATE todos SET text = $1, done = $2 WHERE id = $3")
            .bind(&self.text)
            .bind(&self.done)
            .bind(self.id)
            .execute(pool)
            .await.expect("Couldn't update Todo in DB");
    }

    pub async fn delete(&self, pool: &PgPool) {
        sqlx::query("DELETE FROM todos WHERE id = $1")
            .bind(self.id)
            .execute(pool)
            .await.expect("Couldn't delete Todo in DB");
    }
}
