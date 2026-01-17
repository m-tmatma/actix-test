use actix_web::{web, get, post, App, HttpServer, Responder, HttpResponse};
use serde::{Serialize, Deserialize};

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

#[get("/html")]
async fn render_html() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body("<h1>Hello, HTML!</h1>")
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    id: u32,
    username: String,
    email: String,
}

#[get("/users/{id}")]
async fn get_user_json(id: web::Path<u32>) -> impl Responder {
    let user_id = id.into_inner();
    let user = User {
        id: user_id,
        username: format!("User{}", user_id),
        email: format!("user{}@example.com", user_id),
    };

    HttpResponse::Ok().json(user)
}

#[post("/users")]
async fn create_user_json(user: web::Json<User>) -> impl Responder {
    println!("Received user: {:?}", user);
    HttpResponse::Created().json(user.into_inner())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server");
    HttpServer::new(|| App::new()
        .service(hello)
        .service(render_html)
        .service(get_user_json)
        .service(create_user_json)
        .route("/greet/{name}", web::get().to(greet_name))
        .route("/user/{username}/{id}", web::get().to(user_info)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
