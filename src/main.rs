use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn heath_check() -> impl Responder{
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            //.route("/{name}", web::get().to(greet))
            .route("/heath_check", web::get().to(heath_check))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
