use crate::model::todo;
use crate::repository::pg::Repository;
use actix_web::{delete, get, http, patch, post, web, HttpResponse, ResponseError};
use serde::Deserialize;
use serde_json::json;
use strum_macros::Display;
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, IntoParams)]
pub struct TodoIdentifier {
    todo_id: i32,
}

#[derive(Debug, Display, ToSchema)]
pub enum TodoError {
    TodoNotFound,
    TodoUpdateFailure,
    TodoDeleteFailure,
    TodoCreationFailure,
    BadTodoRequest,
}

impl ResponseError for TodoError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(http::header::ContentType::json())
            .body(json!({"message":self.to_string()}).to_string())
    }

    fn status_code(&self) -> http::StatusCode {
        match self {
            TodoError::TodoNotFound => http::StatusCode::NOT_FOUND,
            TodoError::TodoUpdateFailure => http::StatusCode::CONFLICT,
            TodoError::TodoCreationFailure => http::StatusCode::BAD_REQUEST,
            TodoError::BadTodoRequest => http::StatusCode::BAD_REQUEST,
            TodoError::TodoDeleteFailure => http::StatusCode::CONFLICT,
        }
    }
}

#[utoipa::path(
    params(
        ("todo_id", description = "Unique id of Todo")
    ),
    responses(
        (status = 200, description = "Get a Todo Item", body = Todo)
    )
)]
#[get("/todos/{todo_id}")]
pub async fn get_todo(
    repository: web::Data<Repository>,
    todo_id: web::Path<TodoIdentifier>,
) -> Result<web::Json<todo::Todo>, TodoError> {
    let todo = repository.get_todo(todo_id.todo_id).await;
    match todo {
        Ok(todo) => Ok(web::Json(todo)),
        Err(e) => Err(e),
    }
}

#[utoipa::path(
    responses(
        (status = 200, description = "List current todo items", body = [Todo])
    )
)]
#[get("/todos")]
pub async fn get_todos(
    repository: web::Data<Repository>,
) -> Result<web::Json<Vec<todo::Todo>>, TodoError> {
    let todos = repository.get_todos().await;

    match todos {
        Ok(todos) => Ok(web::Json(todos)),
        Err(e) => Err(e),
    }
}

#[utoipa::path(
    request_body = model::todo::CreateTodo,
    responses(
        (status = 200, description = "Create a Todo", body = Todo)
    )
)]
#[post("/todos")]
pub async fn create_todo(
    repository: web::Data<Repository>,
    todo: web::Json<todo::CreateTodo>,
) -> Result<web::Json<todo::Todo>, TodoError> {
    let todo = repository.create_todo(todo.into_inner()).await;

    match todo {
        Ok(todo) => Ok(web::Json(todo)),
        Err(e) => Err(e),
    }
}

#[utoipa::path(
    request_body = model::todo::UpdateTodo,
    params(
        ("todo_id", description = "Unique id of Todo")
    ),
    responses(
        (status = 200, description = "Create a Todo", body = Todo)
    )
)]
#[patch("/todos/{todo_id}")]
pub async fn update_todo(
    repository: web::Data<Repository>,
    todo: web::Json<todo::UpdateTodo>,
    todo_id: web::Path<TodoIdentifier>,
) -> Result<web::Json<todo::Todo>, TodoError> {
    let todo = repository
        .update_todo(todo_id.todo_id, todo.into_inner())
        .await;

    match todo {
        Ok(todo) => Ok(web::Json(todo)),
        Err(e) => Err(e),
    }
}

#[utoipa::path(
    params(
        ("todo_id", description = "Unique id of Todo")
    ),
    responses(
        (status = 200, description = "Create a Todo", body = Todo)
    )
)]
#[delete("/todos/{todo_id}")]
pub async fn delete_todo(
    repository: web::Data<Repository>,
    todo_id: web::Path<TodoIdentifier>,
) -> Result<web::Json<()>, TodoError> {
    let todo = repository.delete_todo(todo_id.todo_id).await;

    match todo {
        Ok(todo) => Ok(web::Json(todo)),
        Err(e) => Err(e),
    }
}
