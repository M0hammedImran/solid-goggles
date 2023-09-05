use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use models::{NewTodo, Todo, UpdateTodo};
use schema::todos::dsl::*;
use std::env;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_post(conn: &mut PgConnection, todo_title: &str, todo_body: &str) -> Todo {
    let new_todo = NewTodo {
        title: todo_title,
        body: todo_body,
    };

    diesel::insert_into(todos)
        .values(&new_todo)
        .returning(Todo::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn update_post(conn: &mut PgConnection, todo_id: &i32, todo: &UpdateTodo) -> Todo {
    diesel::update(todos.filter(id.eq(todo_id)))
        .set(todo)
        .returning(Todo::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn find_post(conn: &mut PgConnection, todo_id: &i32) -> Result<Todo, diesel::result::Error> {
    todos.find(todo_id).select(Todo::as_select()).first(conn)
}

pub fn find_posts(conn: &mut PgConnection) -> Result<Vec<Todo>, diesel::result::Error> {
    todos.limit(5).select(Todo::as_select()).load(conn)
}
