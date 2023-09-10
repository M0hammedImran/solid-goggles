mod api;
mod model;
mod repository;

use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use api::todo;
use repository::pg::Repository;
use std::env;
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

// fn health_handler() -> impl Responder {
//     const MESSAGE: &str = "Hello World";

//     HttpResponse::Ok().json(json!({"status": "success","message": MESSAGE}))
// }

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

    #[derive(OpenApi)]
    #[openapi(
        paths(
            api::todo::get_todo,        
            api::todo::get_todos,        
            api::todo::create_todo,        
            api::todo::update_todo,        
        ),
        components(
            schemas(
                model::todo::Todo, 
                model::todo::CreateTodo, 
                model::todo::UpdateTodo
            )
        ),
        tags(
            (name = "todo", description = "Todo management endpoints.")
        ),
        modifiers(&SecurityAddon)
    )]
    struct ApiDoc;

    struct SecurityAddon;

    impl Modify for SecurityAddon {
        fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
            let components = openapi.components.as_mut().unwrap(); // we can unwrap safely since there already is components registered.
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("todo_apikey"))),
            )
        }
    }

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
        let app_data = Data::new(repository.clone());

        App::new()
            .wrap(Logger::default())
            .app_data(app_data)
            // .route("/api/v1/health", health_handler)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .service(Redoc::with_url("/redoc", openapi.clone()))
            .service(todo::get_todo)
            .service(todo::get_todos)
            .service(todo::create_todo)
            .service(todo::update_todo)
            .service(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
        // .wrap(ErrorHandlers::new().handler(StatusCode::INTERNAL_SERVER_ERROR, add_error_header))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
