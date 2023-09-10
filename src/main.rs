mod api;
mod model;
mod repository;

use actix_web::{get, middleware::Logger, web::Data, App, HttpResponse, HttpServer, Responder};
use api::todo;
use repository::pg::Repository;
use serde_json::json;
use std::env;

#[get("/api/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Hello World";

    HttpResponse::Ok().json(json!({"status": "success","message": MESSAGE}))
}

// fn add_error_header<B>(
//     mut res: dev::ServiceResponse<B>,
// ) -> Result<ErrorHandlerResponse<B>, actix_web::Error> {
//     res.response_mut().headers_mut().insert(
//         header::CONTENT_TYPE,
//         header::HeaderValue::from_static("Error"),
//     );

//     Ok(ErrorHandlerResponse::Response(res.map_into_left_body()))
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=debug");
    }

    env_logger::builder().init();

    println!("🚀 Server started successfully: http://0.0.0.0:8080");

    let database_url = match env::var("DATABASE_URL") {
        Ok(var) => var,
        Err(e) => panic!("{}", e.to_string()),
    };

    let repository = match Repository::init(String::from("todos"), database_url).await {
        Ok(t) => t,
        Err(e) => panic!("{:?}", e),
    };

    HttpServer::new(move || {
        let app_data = Data::new(repository.clone());

        App::new()
            .wrap(Logger::default())
            .app_data(app_data)
            .service(health_checker_handler)
            .service(todo::get_todo)
            .service(todo::get_todos)
            .service(todo::create_todo)
            .service(todo::update_todo)
        // .wrap(ErrorHandlers::new().handler(StatusCode::INTERNAL_SERVER_ERROR, add_error_header))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
