use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

mod database;

#[derive(Deserialize)]
struct ArticleQuery {
    name: String,
}

#[get("/greet")]
async fn greet(query: web::Query<ArticleQuery>) -> impl Responder {
    database::mysql::query().await;
    HttpResponse::Ok().body(format!("name is {}", query.name))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/v1")
                .service(greet)
                .route("/sample", web::get().to(|| async { "Hello World!" })),
        )
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
