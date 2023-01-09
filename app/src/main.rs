use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct ArticleQuery {
    name: String,
}

#[get("/greet")]
async fn greet(query: web::Query<ArticleQuery>) -> impl Responder {
    HttpResponse::Ok().body(format!("name is {}", query.name))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/sample", web::get().to(|| async { "Hello World!" }))
            .service(greet)
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
