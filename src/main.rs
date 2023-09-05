use rust_todo::*;

fn main() {
    let connection = &mut establish_connection();

    // create_post(connection, "New Todo 3 ", "Description");
    // match find_posts(connection) {
    //     Ok(todos) => {
    //         for todo in todos {
    //             println!("-----------");
    //             println!("{}", todo.title);
    //             println!("{}", todo.body);
    //             println!("-----------\n");
    //         }
    //     }
    //     Err(_) => panic!("Error While Fetching Todos"),
    // }

    match find_post(connection, &2) {
        Ok(todo) => {
            println!("-----------");
            println!("{}", todo.title);
            println!("{}", todo.body);
            println!("-----------\n");
        }
        Err(_) => panic!("Error While Fetching Todos"),
    }

    // update_post(
    //     connection,
    //     &1,
    //     &models::UpdateTodo {
    //         is_done: Some(&true),
    //         title: None,
    //         body: None,
    //     },
    // );
}
