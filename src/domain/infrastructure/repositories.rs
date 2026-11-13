use async_trait::async_trait;
use sqlx::PgPool;
use crate::domain::models::{Task, TaskError};
use crate::domain::repositories::TaskRepository;
use uuid::Uuid;

pub struct PgTaskRepository {
    pool: PgPool,
}

impl PgTaskRepository {
    pub fn new(pool: PgPool) -> Self {
        PgTaskRepository { pool }
    }
}

#[async_trait]
impl TaskRepository for PgTaskRepository {
    async fn create(&self, task: Task) -> Result<Task, TaskError> {
        sqlx::query_as!(
            Task,
            r#"
            INSERT INTO tasks (id, title, description, status)
            VALUES ($1, $2, $3, $4)
            RETURNING id, title, description, status as "status: _"
            "#,
            task.id,
            task.title,
            task.description,
            task.status as _
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            log::error!("Failed to create task: {}", e);
            TaskError::NotFound
        })
    }

    async fn find_all(&self) -> Result<Vec<Task>, TaskError> {
        sqlx::query_as!(
            Task,
            r#"
            SELECT id, title, description, status as "status: _"
            FROM tasks
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            log::error!("Failed to fetch tasks: {}", e);
            TaskError::NotFound
        })
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Task>, TaskError> {
        sqlx::query_as!(
            Task,
            r#"
            SELECT id, title, description, status as "status: _"
            FROM tasks
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            log::error!("Failed to fetch task by id: {}", e);
            TaskError::NotFound
        })
    }

    async fn find_by_title(&self, title: &str) -> Result<Option<Task>, TaskError> {
        sqlx::query_as!(
            Task,
            r#"
            SELECT id, title, description, status as "status: _"
            FROM tasks
            WHERE title = $1
            "#,
            title
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            log::error!("Failed to fetch task by title: {}", e);
            TaskError::NotFound
        })
    }

    async fn update(&self, task: Task) -> Result<Task, TaskError> {
        sqlx::query_as!(
            Task,
            r#"
            UPDATE tasks
            SET title = $2, description = $3, status = $4
            WHERE id = $1
            RETURNING id, title, description, status as "status: _"
            "#,
            task.id,
            task.title,
            task.description,
            task.status as _
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            log::error!("Failed to update task: {}", e);
            TaskError::NotFound
        })
    }

    async fn delete(&self, id: Uuid) -> Result<(), TaskError> {
        sqlx::query!(
            r#"
            DELETE FROM tasks
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| {
            log::error!("Failed to delete task: {}", e);
            TaskError::NotFound
        })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::postgres::PgPoolOptions;
    use std::env;

    async fn setup_test_pool() -> PgPool {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .expect("Failed to connect to Postgres")
    }

    #[actix_rt::test]
    async fn test_create_and_find_task() {
        let pool = setup_test_pool().await;
        let repo = PgTaskRepository::new(pool);

        let task = Task::new("Test Task".to_string(), "Test Description".to_string()).unwrap();
        let created_task = repo.create(task.clone()).await.unwrap();

        let found_task = repo.find_by_id(created_task.id).await.unwrap();
        assert_eq!(found_task, Some(task));
    }
}