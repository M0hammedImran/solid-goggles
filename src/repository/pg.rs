use crate::model::todo;
use crate::api::todo::TodoError;
use log::error;
use sqlx::{
    postgres,
    PgPool,
};
use std::str::FromStr;

#[derive(Clone)]
pub struct Repository {
    client: PgPool,
}

impl Repository {
    pub async fn init( uri: String) -> Result<Repository, String> {
        let options = postgres::PgConnectOptions::from_str(uri.as_str()).unwrap().clone();

        match postgres::PgPool::connect_with(options).await {
            Ok(client) => Ok(Repository { client }),
            Err(err) => {
                error!("{:?}", err);
                Err(err.to_string())
            }
        }
    }

    pub async fn get_todo(&self, todo_id: i32) -> Result<todo::Todo, TodoError> {
        let todo = sqlx::query_as!(
            todo::Todo, 
            "SELECT id, title, body, is_done, created_at, updated_at FROM public.todos where id = $1", 
            todo_id)
            .fetch_one(&self.client).await;

        match todo {
            Ok(todo) => Ok(todo),
            Err(error) => {
                println!("{:?}", error.to_string());
                Err(TodoError::TodoNotFound)
            }
        }
    }
    pub async fn get_todos(&self) -> Result<Vec<todo::Todo>, TodoError> {
        let todos = sqlx::query_as!(
            todo::Todo, 
            "SELECT id, title, body, is_done, created_at, updated_at FROM public.todos"
        ).fetch_all(&self.client).await;

        match todos {
            Ok(todos) => Ok(todos),
            Err(error) => {
                println!("{:?}", error.to_string());
                Err(TodoError::BadTodoRequest)
            }
        }
    }
    pub async fn create_todo(&self, todo: todo::CreateTodo) -> Result<todo::Todo, TodoError> {
        let todo = sqlx::query_as!(
            todo::Todo, 
            "insert into public.todos
                (title, body, is_done, created_at, updated_at)
            values($1, $2, false, timezone('UTC', now()), timezone('UTC', now())) 
                returning id, body, title, created_at, updated_at, is_done;", 
            todo.title, 
            todo.body 
        ).fetch_one(&self.client).await;

        match todo {
            Ok(todo) => Ok(todo),
            Err(error) => {
                println!("{:?}", error.to_string());
                Err(TodoError::TodoCreationFailure)
            }
        }
    }

    pub async fn update_todo(&self,todo_id: i32, _todo: todo::UpdateTodo) -> Result<todo::Todo, TodoError> {
        let updated_todo = sqlx::query_as!(
            todo::Todo, 
            "update public.todos
            set
                title = '', 
                body = '', 
                is_done = false, 
                updated_at = timezone('UTC', now())
            where id = $1
                returning id, body, title, created_at, updated_at, is_done;", 
            todo_id
        ).fetch_one(&self.client).await;

        match updated_todo {
            Ok(output) => Ok(output),
            Err(error) => {
                println!("{:?}", error.to_string());
                Err(TodoError::TodoUpdateFailure)
            }
        }
    }

    pub async fn delete_todo(&self,todo_id: i32) -> Result<(), TodoError> {
        let updated_todo = sqlx::query_as!(
            todo::Todo, 
            "DELETE from public.todos
            where id = $1",
            todo_id
        ).fetch_one(&self.client).await;

        match updated_todo {
            Ok(_) => Ok(()),
            Err(error) => {
                println!("{:?}", error.to_string());
                Err(TodoError::TodoDeleteFailure)
            }
        }
    }
}
