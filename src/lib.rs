use actix_web::{web, App, HttpRequest, HttpServer, HttpResponse, Responder};
use actix_web::dev::Server;
use std::net::TcpListener;

async fn greet(req: HttpRequest) -> impl Responder {
    let name: &str = req.match_info().get("name").unwrap_or("World");
    format!("Hello, {name}!")
}

async fn health() -> impl Responder {
    HttpResponse::Ok()
}

pub async fn run(listener: TcpListener) -> Result<Server, std::io::Error>{
    let server = HttpServer::new(|| {
            App::new()
                .route("/health", web::get().to(health))
                .route("/", web::get().to(greet))
                .route("/{name}", web::get().to(greet))
        })
        .listen(listener).expect("Failed to listen")
        .run();
    Ok(server)
}
