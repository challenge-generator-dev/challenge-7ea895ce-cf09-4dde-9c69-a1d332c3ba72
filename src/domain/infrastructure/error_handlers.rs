// TODO(reto): archivo stubbeado; implementa los cuerpos.
use actix_web::{error, HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

impl From<crate::domain::models::TaskError> for actix_web::Error {
    fn from(err: crate::domain::models::TaskError) -> Self {
      todo!("TODO(reto): from")
    }
}

// TODO(reto): Implementar el manejo de errores HTTP
// El estudiante debe implementar la conversión de TaskError a HttpResponse
// con los códigos de estado HTTP correctos:
// - TitleExists -> 409 Conflict
// - DescriptionTooShort -> 400 Bad Request
// - NotFound -> 404 Not Found
pub fn handle_task_error(err: crate::domain::models::TaskError) -> HttpResponse {
  todo!("TODO(reto): TODO")
}