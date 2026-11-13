use crate::domain::models::{Task, TaskError, CreateTaskDto, UpdateTaskDto};
use crate::domain::repositories::TaskRepository;
use uuid::Uuid;

pub struct TaskService<R: TaskRepository> {
    repository: R,
}

impl<R: TaskRepository> TaskService<R> {
    pub fn new(repository: R) -> Self {
        TaskService { repository }
    }

    pub async fn create_task(&self, create_dto: CreateTaskDto) -> Result<Task, TaskError> {
        if let Some(_) = self.repository.find_by_title(&create_dto.title).await? {
            return Err(TaskError::TitleExists);
        }

        let task = Task::new(create_dto.title, create_dto.description)?;
        self.repository.create(task).await
    }

    pub async fn get_all_tasks(&self) -> Result<Vec<Task>, TaskError> {
        self.repository.find_all().await
    }

    pub async fn get_task_by_id(&self, id: Uuid) -> Result<Option<Task>, TaskError> {
        self.repository.find_by_id(id).await
    }

    pub async fn update_task(
        &self,
        id: Uuid,
        update_dto: UpdateTaskDto,
    ) -> Result<Task, TaskError> {
        let mut task = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or(TaskError::NotFound)?;

        if let Some(title) = &update_dto.title {
            if let Some(existing_task) = self.repository.find_by_title(title).await? {
                if existing_task.id != id {
                    return Err(TaskError::TitleExists);
                }
            }
        }

        task.update(update_dto)?;
        self.repository.update(task).await
    }

    pub async fn delete_task(&self, id: Uuid) -> Result<(), TaskError> {
        self.repository.delete(id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::repositories::tests::MockTaskRepo;
    use mockall::predicate::*;

    #[actix_rt::test]
    async fn test_create_task_success() {
        let mut mock_repo = MockTaskRepo::new();
        mock_repo
            .expect_find_by_title()
            .with(eq("New Task"))
            .times(1)
            .returning(|_| Ok(None));
        mock_repo
            .expect_create()
            .times(1)
            .returning(|task| Ok(task));

        let service = TaskService::new(mock_repo);
        let create_dto = CreateTaskDto {
            title: "New Task".to_string(),
            description: "A valid description with more than 10 chars".to_string(),
        };

        let result = service.create_task(create_dto).await;
        assert!(result.is_ok());
    }

    #[actix_rt::test]
    async fn test_create_task_title_exists() {
        let mut mock_repo = MockTaskRepo::new();
        mock_repo
            .expect_find_by_title()
            .with(eq("Existing Task"))
            .times(1)
            .returning(|_| Ok(Some(Task::new("Existing Task".to_string(), "Description".to_string()).unwrap())));

        let service = TaskService::new(mock_repo);
        let create_dto = CreateTaskDto {
            title: "Existing Task".to_string(),
            description: "A valid description".to_string(),
        };

        let result = service.create_task(create_dto).await;
        assert!(matches!(result, Err(TaskError::TitleExists)));
    }
}