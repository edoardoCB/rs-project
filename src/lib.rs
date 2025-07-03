use actix_web::{web, App, HttpRequest, HttpServer, HttpResponse, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
    let name: &str = req.match_info().get("name").unwrap_or("World");
    format!("Hello, {name}!")
}

async fn health() -> impl Responder {
    HttpResponse::Ok()
}

pub async fn run() -> Result<(), std::io::Error>{
    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health))
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
        .bind("127.0.0.1:8080").expect("Can not bind to 127.0.0.1:8080")
        .run()
        .await
}
