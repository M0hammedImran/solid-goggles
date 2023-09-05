use diesel::{data_types::PgTimestamp, prelude::*};

use crate::schema::todos;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = todos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub is_done: bool,
    pub created_at: PgTimestamp,
    pub updated_at: PgTimestamp,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = todos)]
pub struct NewTodo<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = todos)]
pub struct UpdateTodo<'a> {
    pub title: Option<&'a str>,
    pub body: Option<&'a str>,
    pub is_done: Option<&'a bool>,
}
