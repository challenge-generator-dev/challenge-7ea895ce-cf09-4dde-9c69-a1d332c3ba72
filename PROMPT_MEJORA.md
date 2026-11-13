# Prompt para Mejorar el Codigo Base

Copia y pega el contenido del bloque de abajo en un asistente de IA (Claude, ChatGPT)
para obtener el proyecto completo y arrancable. Guarda los archivos que te devuelva en la
carpeta del repo, respetando exactamente las rutas.

Si preferis trabajar en tu editor con un agente local (Claude Code, Cursor, Copilot), usa `AGENTS.md` en vez de este archivo: dice lo mismo pero para que escriba los archivos en disco.

## Las dos reglas que no se negocian

1. **Completa el boilerplate.** Todo lo que el proyecto necesita para compilar y arrancar: manifiesto de dependencias, punto de entrada, configuracion, capa de interfaz, y las capas del patron arquitectonico declarado. Eso es andamiaje y es tu trabajo.
2. **NO resuelvas el reto.** Los entregables de las fases son el trabajo de la persona. El hueco pedagogico se deja como esta: el proyecto arranca, pero lo que el reto pide implementar NO esta implementado.

Dicho de otra forma: si algo impide compilar, arreglalo. Si algo es logica de negocio incompleta, validaciones ausentes, un secreto hardcodeado o un patron mejorable, dejalo exactamente como esta — es lo que la persona tiene que encontrar.

## Superficie de practica — NO resuelvas

Estos archivos SON el ejercicio de la persona. No los implementes; deja stubs.

- `src/domain/infrastructure/routes.rs` — La fase "Definición y creación de endpoints básicos" se implementa aqui: este archivo es el ejercicio.
- `src/domain/infrastructure/error_handlers.rs` — La fase "Actualización de tareas y manejo de errores" se implementa aqui: este archivo es el ejercicio.

## Como saber que terminaste

```bash
cargo check
```

Ese comando corriendo sin errores es la definicion de "listo".

---

```
## Briefing del reto (autoridad)
Este bloque manda sobre los archivos adjuntos. El stack y el rol salen de AQUÍ, no de un topic genérico ni de markdown placeholder.

### Perfil
Chapter Backend, Especialidad Desarrollador, Tecnología Rust, Senior L2

### Brecha de conocimiento
Construir una API REST en Rust con Actix Web para gestionar tareas, Implementa endpoints REST en Rust con Actix Web para crear, listar y actualizar tareas

### Misión / candidato
Exponer la gestion de tareas por HTTP

### Reto
- Tema: api-rest-tareas-rust
- Seniority: senior-l2
- Tipo: practical
- Título: Implementación de API REST de gestión de tareas en Rust
- Tiempo estimado: 8 horas

### Fases (trabajo del HUMANO — PROHIBIDO completarlas)
No implementes estos entregables. Dejalos como hueco pedagógico. El asistente solo materializa el proyecto arrancable para que el participante pueda trabajar.
- Fase 1: Definición y creación de endpoints básicos — objetivo: Implementar los endpoints REST para crear y listar tareas. — entregable (NO resolver): Endpoints funcionales para crear y listar tareas, con validaciones básicas.
- Fase 2: Actualización de tareas y manejo de errores — objetivo: Implementar el endpoint para actualizar tareas y mejorar el manejo de errores. — entregable (NO resolver): Endpoint funcional para actualizar tareas, con mejoras en el manejo de errores.
- Fase 3: Refactorización y optimización — objetivo: Refactorizar el código para mejorar su legibilidad y rendimiento, y agregar funcionalidades adicionales si es necesario. — entregable (NO resolver): Código refactorizado y optimizado, con posibles funcionalidades adicionales implementadas.

Eres un asistente experto en análisis, corrección y generación de archivos de cualquier tipo:
código fuente, documentación, hojas de cálculo, documentos Word, configuraciones, entre otros.
Voy a enviarte una cadena de texto que contiene uno o más archivos. Cada archivo está delimitado por un marcador con el siguiente formato:
// === ARCHIVO: ruta/del/archivo.extension ===
o también puede aparecer como:
## === ARCHIVO: ruta/del/archivo.extension ===
Lo que sigue al marcador puede ser:

El contenido real del archivo (código, texto, YAML, etc.)
Una descripción en lenguaje natural de lo que debe contener el archivo


TU TAREA
PASO 0 — ¿Esto es un proyecto o una carcasa?
Antes de extraer archivos, leé el Briefing (si está) y diagnosticá el adjunto.

Es CARCASA si ocurre CUALQUIERA de estas:
- No hay manifiesto de dependencias del stack del briefing (manifest.json de VTEX IO / package.json / pom.xml / build.gradle / requirements.txt / go.mod / *.tf / *.csproj, según corresponda)
- Hay un "binario" que en realidad es un comentario ("no puede ser mostrado como texto plano", placeholder .fig/.docx vacío)
- Los markdowns ya completan entregables de fases posteriores ("se implementó fade-in", lista de áreas ya resuelta)

Si es CARCASA:
- MATERIALIZÁ un proyecto que arranca en el stack del briefing (VTEX IO Store Framework, Angular, Terraform, pytest, Nest, etc.). Incluí manifiesto, punto de entrada y capa de interfaz reales.
- NO copies los markdowns de "solución" como si fueran el producto. Son ruido de generación.
- NO resuelvas las fases del briefing (están marcadas PROHIBIDO). Dejá el hueco pedagógico: el flujo existe, las microinteracciones/calidad/infra que el reto pide NO están hechas.
- Después seguí al PASO 5 (entrega de archivos).

Si es un proyecto REAL (manifiesto + código que compila o arranca):
- Seguí PASO 1 en adelante. 🔴 compilación sí. 🟡 pedagógico no.

PASO 1 — Detección y extracción
Identifica todos los archivos presentes en la cadena. Para cada archivo extrae:

Su ruta completa (ej: src/main/java/com/pragma/Service.java)
Su contenido o descripción

PASO 2 — Clasificación por tipo
Clasifica cada archivo en una de estas categorías:
A) Código fuente (Java, Python, TypeScript, JavaScript, Kotlin, etc.)
B) Configuración / documentación (YAML, properties, Markdown, JSON, txt, etc.)
C) Excel (.xlsx, .xls, .csv)
D) Word (.docx, .doc)
E) Otro tipo de archivo binario o especial
PASO 3 — Clasificación de errores en código fuente

Objetivo prioritario: que el proyecto compile. No corrijas flujo de negocio ni lógica funcional.

Antes de modificar cualquier archivo de código fuente, clasifica cada problema encontrado en una de estas dos categorías:
🔴 ERROR DE COMPILACIÓN — corregir siempre
Son errores que impiden que el proyecto arranque, sin valor pedagógico:

Import faltante o incorrecto
Clase, método o variable referenciada que no existe en ningún archivo del proyecto
Error de sintaxis
Anotación con atributos inválidos
Dependencia ausente en pom.xml, package.json, etc.
Archivo referenciado que no existe y debe ser creado con implementación mínima

→ CORREGIR estos errores.
🟡 PROBLEMA FUNCIONAL O DE CALIDAD — preservar siempre
Son problemas que no impiden compilar. Pueden ser intencionales para el aprendizaje:

Clave secreta hardcodeada ("secret", "password123")
API deprecada que funciona pero tiene reemplazo moderno
Lógica de negocio incorrecta o incompleta
Código redundante o de baja legibilidad
Falta de validaciones en flujo de negocio
Patrones de diseño incorrectos pero funcionales
Concurrencia no segura
Configuración funcional pero no óptima

→ PRESERVAR tal cual. No corregir, no mejorar, no comentar.
PASO 4 — Procesamiento según tipo de archivo
Tipo A — Código fuente
Aplica únicamente las correcciones clasificadas como 🔴 ERROR DE COMPILACIÓN.
No alteres ningún elemento clasificado como 🟡 PROBLEMA FUNCIONAL O DE CALIDAD.
Si falta un archivo referenciado, créalo con la implementación mínima necesaria para compilar.
Tipo B — Configuración / documentación
Extrae el contenido tal cual, sin modificaciones salvo errores evidentes de sintaxis
(ej: YAML mal indentado).
Tipo C — Excel (.xlsx)
Si viene con contenido real, genera el archivo respetando ese contenido.
Si viene con descripción en lenguaje natural, genera un archivo Excel funcional con:

Fila de encabezados en negrita con color de fondo distintivo
Columnas con ancho ajustado al contenido
Tipos de dato correctos por columna
Validaciones si la descripción lo indica
Hojas nombradas descriptivamente si hay más de una
Filas de ejemplo si no hay datos reales

Tipo D — Word (.docx)
Si viene con contenido real, genera el archivo respetando ese contenido.
Si viene con descripción en lenguaje natural, genera un documento Word funcional con:

Estilos de título (Título 1, Título 2) para jerarquía de secciones
Fuente legible (Calibri o equivalente), tamaño 11-12pt para cuerpo
Márgenes estándar
Tabla de contenido si tiene múltiples secciones
Tablas con encabezados en negrita si aplica

Tipo E — Otro
Genera el archivo con el contenido o estructura más apropiada según la descripción.
PASO 5 — Entrega de archivos
Entregá cada archivo con su ruta completa, respetando exactamente la estructura de rutas
indicada por los marcadores: un bloque por archivo, precedido por su ruta. No generes ZIPs
ni archivos adjuntos.
La entrega debe incluir:

Archivos de código con únicamente los errores de compilación corregidos
Archivos de configuración y documentación sin cambios
Archivos nuevos creados para resolver dependencias de compilación faltantes
Archivos Excel y Word generados desde descripción

IMPORTANTE: Entregá los archivos completos al finalizar. No preguntes si el usuario
quiere recibirlos. Simplemente entregalos; No debes desplegar en el chat el resumen de lo que arreglaste, solo entregá los archivos.

REGLAS IMPORTANTES

No omitas ningún archivo aunque no tenga errores ni modificaciones
Respeta los nombres y rutas exactas indicadas por los marcadores
Si un archivo no tiene marcador claro, infiere el nombre desde su contenido
Si la cadena contiene solo documentación, placeholders o binarios fake, NO la reproduzcas:
aplicá PASO 0 (materializar el proyecto del briefing). Reproducir la carcasa es un fallo.
No agregues texto después del último archivo entregado
No preguntes si el usuario quiere los archivos: simplemente entregalos siempre
Si detectas que falta un archivo de configuración necesario para compilar
(pom.xml, package.json, requirements.txt, build.gradle, etc.), créalo e inclúyelo
inferiendo su contenido desde los imports y frameworks detectados en el código
Nunca corrijas problemas 🟡 aunque parezcan obvios o fáciles de mejorar.
El participante que recibirá este proyecto los debe encontrar y resolver él mismo.


INPUT
Aquí está la cadena con los archivos:

// === ARCHIVO: Cargo.toml ===
[package]
name = "actix-web-skeleton"
version = "0.1.0"
edition = "2021"

[dependencies]
actix-web = "4.19.0"
anyhow = "1.0.104"
async-trait = "0.1.92"
dotenv = "0.15.0"
serde = { version = "1.0.229", features = ["derive"] }
sqlx = { version = "0.9.0", features = ["runtime-tokio", "macros"] }
thiserror = "1.0.69"
validator = "0.21.0"
log = "0.4.35"
mockall = "0.15.0"
uuid = { version = "1.29.2", features = ["v4", "serde"] }

[dev-dependencies]
actix-rt = "2.15.0"

// === ARCHIVO: src/main.rs ===
pub mod domain;

use actix_web::web;
use crate::domain::infrastructure::routes::config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .configure(config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// === ARCHIVO: src/contracts/types.rs ===
// Sample contract types
pub trait DomainContract {
    fn validate(&self) -> bool;
}

// === ARCHIVO: src/domain/mod.rs ===
pub mod config;
pub mod models;
pub mod repositories;
pub mod application;
pub mod infrastructure;
pub mod web;

// === ARCHIVO: src/contracts/mod.rs ===
// Contracts root module
pub mod types;

// === ARCHIVO: src/domain/config/mod.rs ===
use serde::Deserialize;
use std::env;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Missing environment variable: {0}")]
    MissingEnvVar(String),
    #[error("Invalid environment variable: {0}")]
    InvalidEnvVar(String),
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub database_url: String,
    pub server_port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenv::dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .map_err(|_| ConfigError::MissingEnvVar("DATABASE_URL".to_string()))?;

        let server_port = env::var("SERVER_PORT")
            .map_err(|_| ConfigError::MissingEnvVar("SERVER_PORT".to_string()))
            .and_then(|port| {
                port.parse::<u16>()
                    .map_err(|_| ConfigError::InvalidEnvVar("SERVER_PORT".to_string()))
            })?;

        Ok(Config {
            database_url,
            server_port,
        })
    }
}

// === ARCHIVO: src/domain/models.rs ===
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

// === ARCHIVO: src/domain/repositories.rs ===
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

// === ARCHIVO: src/domain/application/services.rs ===
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

// === ARCHIVO: src/domain/infrastructure/repositories.rs ===
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

// === ARCHIVO: src/domain/infrastructure/routes.rs ===
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

// === ARCHIVO: src/domain/infrastructure/error_handlers.rs ===
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

// === ARCHIVO: src/domain/web/mod.rs ===
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

// === ARCHIVO: src/domain/application/mod.rs ===
pub mod services;


// === ARCHIVO: src/domain/infrastructure/mod.rs ===
pub mod error_handlers;
pub mod repositories;
pub mod routes;

```
