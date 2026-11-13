use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

pub mod types {
    pub use actix_web::web::{Data, Json, Path, Query};
    pub use actix_web::{HttpRequest, HttpResponse};
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        ApiResponse {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(message: String) -> Self {
        ApiResponse {
            success: false,
            data: None,
            error: Some(message),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationParams {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

impl Default for PaginationParams {
    fn default() -> Self {
        PaginationParams {
            page: Some(1),
            limit: Some(10),
        }
    }
}

impl PaginationParams {
    pub fn get_page(&self) -> usize {
        self.page.unwrap_or(1)
    }

    pub fn get_limit(&self) -> usize {
        self.limit.unwrap_or(10).min(100)
    }

    pub fn get_offset(&self) -> usize {
        (self.get_page() - 1) * self.get_limit()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListResponse<T> {
    pub items: Vec<T>,
    pub total: usize,
    pub page: usize,
    pub limit: usize,
}

impl<T> ListResponse<T> {
    pub fn new(items: Vec<T>, total: usize, page: usize, limit: usize) -> Self {
        ListResponse {
            items,
            total,
            page,
            limit,
        }
    }
}

pub trait IntoHttpResponse {
    fn into_response(self) -> HttpResponse
    where
        Self: Sized;
}

impl<T> IntoHttpResponse for Result<T, crate::domain::models::TaskError>
where
    T: Serialize,
{
    fn into_response(self) -> HttpResponse {
        match self {
            Ok(data) => HttpResponse::Ok().json(ApiResponse::success(data)),
            Err(err) => {
                let status = match err {
                    crate::domain::models::TaskError::TitleExists => actix_web::http::StatusCode::CONFLICT,
                    crate::domain::models::TaskError::DescriptionTooShort => actix_web::http::StatusCode::BAD_REQUEST,
                    crate::domain::models::TaskError::NotFound => actix_web::http::StatusCode::NOT_FOUND,
                };
                status.json(ApiResponse::error(err.to_string()))
            }
        }
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.app_data(
        web::Json::<serde_json::Value>::configure(|cfg| {
            cfg.error_handler(|(err, _req, _res| {
                actix_web::Error::from(error::ErrorBadRequest(err.to_string()))
            })
        }),
    );
}