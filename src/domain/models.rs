use serde::{Serialize, Deserialize};
use validator::Validate;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum TaskError {
    #[error("Title already exists")]
    TitleExists,
    #[error("Description must be at least 10 characters")]
    DescriptionTooShort,
    #[error("Task not found")]
    NotFound,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Task {
    pub id: Uuid,
    #[validate(length(min = 1, message = "Title cannot be empty"))]
    pub title: String,
    #[validate(length(min = 10, message = "Description must be at least 10 characters"))]
    pub description: String,
    pub status: TaskStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateTaskDto {
    #[validate(length(min = 1, message = "Title cannot be empty"))]
    pub title: String,
    #[validate(length(min = 10, message = "Description must be at least 10 characters"))]
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateTaskDto {
    #[validate(length(min = 1, message = "Title cannot be empty"))]
    pub title: Option<String>,
    #[validate(length(min = 10, message = "Description must be at least 10 characters"))]
    pub description: Option<String>,
    pub status: Option<TaskStatus>,
}

impl Task {
    pub fn new(title: String, description: String) -> Result<Self, TaskError> {
        if description.len() < 10 {
            return Err(TaskError::DescriptionTooShort);
        }

        Ok(Task {
            id: Uuid::new_v4(),
            title,
            description,
            status: TaskStatus::Pending,
        })
    }

    pub fn update(&mut self, update_dto: UpdateTaskDto) -> Result<(), TaskError> {
        if let Some(title) = update_dto.title {
            self.title = title;
        }

        if let Some(description) = update_dto.description {
            if description.len() < 10 {
                return Err(TaskError::DescriptionTooShort);
            }
            self.description = description;
        }

        if let Some(status) = update_dto.status {
            self.status = status;
        }

        Ok(())
    }
}