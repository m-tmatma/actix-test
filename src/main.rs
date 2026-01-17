use actix_web::{web, get, App, HttpServer, Responder, HttpResponse};

#[get("/")]
async fn hello() -> impl Responder {
    "Hello, world!"
}

async fn greet_name(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello, {}!", name))
}

async fn user_info(path: web::Path<(String, u32)>) -> impl Responder {
    let (username, id) = path.into_inner();
    HttpResponse::Ok().body(format!("User: {}, ID: {}", username, id))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server");
    HttpServer::new(|| App::new()
        .service(hello)
        .route("/greet/{name}", web::get().to(greet_name))
        .route("/user/{username}/{id}", web::get().to(user_info)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
