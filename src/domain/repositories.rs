use async_trait::async_trait;
use crate::domain::models::{Task, TaskError};
use uuid::Uuid;

#[async_trait]
pub trait TaskRepository: Send + Sync {
    async fn create(&self, task: Task) -> Result<Task, TaskError>;
    async fn find_all(&self) -> Result<Vec<Task>, TaskError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Task>, TaskError>;
    async fn find_by_title(&self, title: &str) -> Result<Option<Task>, TaskError>;
    async fn update(&self, task: Task) -> Result<Task, TaskError>;
    async fn delete(&self, id: Uuid) -> Result<(), TaskError>;
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use mockall::predicate::*;
    use mockall::*;

    mock! {
        pub TaskRepo {}
        #[async_trait]
        impl TaskRepository for TaskRepo {
            async fn create(&self, task: Task) -> Result<Task, TaskError>;
            async fn find_all(&self) -> Result<Vec<Task>, TaskError>;
            async fn find_by_id(&self, id: Uuid) -> Result<Option<Task>, TaskError>;
            async fn find_by_title(&self, title: &str) -> Result<Option<Task>, TaskError>;
            async fn update(&self, task: Task) -> Result<Task, TaskError>;
            async fn delete(&self, id: Uuid) -> Result<(), TaskError>;
        }
    }
}