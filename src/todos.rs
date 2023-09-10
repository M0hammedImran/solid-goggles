// use crate::schema::todos;

// #[derive(Debug, Serialize, Clone)]
// pub struct Todo {
//     pub id: i32,
//     pub title: String,
//     pub body: String,
//     pub is_done: bool,
// }

// #[derive(Debug)]
// pub struct NewTodo<'a> {
//     pub title: &'a str,
//     pub body: &'a str,
// }

// #[derive(Debug)]
// pub struct UpdateTodo<'a> {
//     pub title: Option<&'a str>,
//     pub body: Option<&'a str>,
//     pub is_done: Option<&'a bool>,
// }

// #[derive(Debug)]
// pub struct CreateTodo {
//     pub description: String,
// }

// impl Todo {
//     pub async fn all(conn: &DbConn) -> QueryResult<Vec<Todo>> {
//         conn.run(|c| todos::table.order(todos::id.desc()).load::<Todo>(c))
//             .await
//     }

//     /// Returns the number of affected rows: 1.
//     pub async fn insert(todo: Todo, conn: &DbConn) -> QueryResult<usize> {
//         conn.run(|c| {
//             let t = NewTodo {
//                 title: &todo.title,
//                 body: &todo.body,
//             };
//             diesel::insert_into(todos::table).values(&t).execute(c)
//         })
//         .await
//     }

//     /// Returns the number of affected rows: 1.
//     pub async fn toggle_with_id(id: i32, conn: &DbConn) -> QueryResult<usize> {
//         conn.run(move |c| {
//             let task = todos::table
//                 .filter(todos::id.eq(id))
//                 .get_result::<Todo>(c)?;
//             let new_status = !task.is_done;
//             let updated_task = diesel::update(todos::table.filter(todos::id.eq(id)));
//             updated_task.set(todos::is_done.eq(new_status)).execute(c)
//         })
//         .await
//     }

//     /// Returns the number of affected rows: 1.
//     pub async fn delete_with_id(id: i32, conn: &DbConn) -> QueryResult<usize> {
//         conn.run(move |c| {
//             diesel::delete(todos::table)
//                 .filter(todos::id.eq(id))
//                 .execute(c)
//         })
//         .await
//     }

//     /// Returns the number of affected rows.
//     #[cfg(test)]
//     pub async fn delete_all(conn: &DbConn) -> QueryResult<usize> {
//         conn.run(|c| diesel::delete(todos::table).execute(c)).await
//     }
// }
