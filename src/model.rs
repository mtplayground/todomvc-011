use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub completed: bool,
    pub ordering: i64,
}

#[cfg(feature = "ssr")]
pub mod db {
    use super::Todo;
    use sqlx::SqlitePool;

    pub async fn get_db() -> SqlitePool {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "sqlite:todos.db".to_string());
        let pool = SqlitePool::connect(&database_url)
            .await
            .expect("Failed to connect to database");

        sqlx::migrate!()
            .run(&pool)
            .await
            .expect("Failed to run migrations");

        pool
    }

    pub async fn get_todos(pool: &SqlitePool) -> Result<Vec<Todo>, sqlx::Error> {
        let rows = sqlx::query_as::<_, (i64, String, bool, i64)>(
            "SELECT id, title, completed, ordering FROM todos ORDER BY ordering ASC, id ASC"
        )
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(|(id, title, completed, ordering)| Todo {
            id,
            title,
            completed,
            ordering,
        }).collect())
    }

    pub async fn add_todo(pool: &SqlitePool, title: String) -> Result<Todo, sqlx::Error> {
        let max_ordering: Option<i64> = sqlx::query_scalar(
            "SELECT MAX(ordering) FROM todos"
        )
        .fetch_one(pool)
        .await?;

        let ordering = max_ordering.unwrap_or(0) + 1;

        let id = sqlx::query(
            "INSERT INTO todos (title, completed, ordering) VALUES (?, FALSE, ?)"
        )
        .bind(&title)
        .bind(ordering)
        .execute(pool)
        .await?
        .last_insert_rowid();

        Ok(Todo {
            id,
            title,
            completed: false,
            ordering,
        })
    }

    pub async fn toggle_todo(pool: &SqlitePool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE todos SET completed = NOT completed WHERE id = ?"
        )
        .bind(id)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn update_todo_title(
        pool: &SqlitePool,
        id: i64,
        title: String,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE todos SET title = ? WHERE id = ?")
            .bind(&title)
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn delete_todo(pool: &SqlitePool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM todos WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn toggle_all_todos(pool: &SqlitePool, completed: bool) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE todos SET completed = ?")
            .bind(completed)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn clear_completed_todos(pool: &SqlitePool) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM todos WHERE completed = TRUE")
            .execute(pool)
            .await?;
        Ok(())
    }
}
