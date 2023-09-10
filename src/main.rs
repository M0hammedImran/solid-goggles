mod api;
mod model;
mod repository;

use actix_web::{get, middleware::Logger, web, App, HttpRequest, HttpServer};
use api::todo;
use repository::pg::Repository;
use std::{collections::HashMap, env};
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

#[get("/api/v1/health")]
async fn health_handler(req: HttpRequest) -> web::Json<HashMap<String, String>> {
    let mut map: HashMap<String, String> = HashMap::new();
    req.headers().into_iter().for_each(|x| {
        map.insert(
            x.0.to_string(),
            x.1.to_str().unwrap_or_default().to_string(),
        );
    });
    web::Json(map)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=debug");
    }

    env_logger::builder().init();

    println!("🚀 Server started successfully: http://0.0.0.0:8080");

    #[derive(OpenApi)]
    #[openapi(
        paths(
            api::todo::get_todo,
            api::todo::get_todos,
            api::todo::create_todo,
            api::todo::update_todo
        ),
        components(
            schemas(
                model::todo::Todo,
                model::todo::CreateTodo,
                model::todo::UpdateTodo,
                api::todo::TodoError
            )
        ),
        tags(
            (name = "todo", description = "Todo management endpoints.")
        ),
    )]
    struct ApiDoc;

    let database_url = match env::var("DATABASE_URL") {
        Ok(var) => var,
        Err(e) => panic!("{}", e.to_string()),
    };

    let repository = match Repository::init(database_url).await {
        Ok(t) => t,
        Err(e) => panic!("{:?}", e),
    };

    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        let app_data = web::Data::new(repository.clone());

        App::new()
            .wrap(Logger::default())
            .app_data(app_data)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .service(Redoc::with_url("/redoc", openapi.clone()))
            .service(health_handler)
            .service(todo::get_todo)
            .service(todo::get_todos)
            .service(todo::create_todo)
            .service(todo::update_todo)
            .service(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
