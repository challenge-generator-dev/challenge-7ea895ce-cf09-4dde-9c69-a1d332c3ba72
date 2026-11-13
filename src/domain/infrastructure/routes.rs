// TODO(reto): archivo stubbeado; implementa los cuerpos.
use actix_web::{web, HttpResponse};
use crate::domain::models::{Task, TaskError};
use crate::domain::application::services::TaskService;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/tasks")
            .route("", web::post().to(create_task))
            .route("", web::get().to(list_tasks))
            .route("/{id}", web::put().to(update_task))
            .route("/{id}", web::delete().to(delete_task))
    );
}

async fn create_task(
    task: web::Json<Task>,
    service: web::Data<TaskService>,
) -> Result<HttpResponse, TaskError> {
  todo!("TODO(reto): create_task")
}

async fn list_tasks(
    service: web::Data<TaskService>,
) -> Result<HttpResponse, TaskError> {
  todo!("TODO(reto): list_tasks")
}

async fn update_task(
    id: web::Path<String>,
    task: web::Json<Task>,
    service: web::Data<TaskService>,
) -> Result<HttpResponse, TaskError> {
  todo!("TODO(reto): update_task")
}

async fn delete_task(
    id: web::Path<String>,
    service: web::Data<TaskService>,
) -> Result<HttpResponse, TaskError> {
  todo!("TODO(reto): delete_task")
}