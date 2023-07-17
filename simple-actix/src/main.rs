// Web microservice for math operations
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};



#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[get("/add/{a}/{b}")]
async fn add(info: web::Path<(i32, i32)>) -> impl Responder {
    let res = simple_actix::add(info.0, info.1);
    HttpResponse::Ok().body(res.to_string())
}

#[get("/subtract/{a}/{b}")]
async fn subtract(info: web::Path<(i32, i32)>) -> impl Responder {
    let res = simple_actix::subtract(info.0, info.1);
    HttpResponse::Ok().body(res.to_string())
}

#[get("/multiply/{a}/{b}")]
async fn multiply(info: web::Path<(i32, i32)>) -> impl Responder {
    let res = simple_actix::multiply(info.0, info.1);
    HttpResponse::Ok().body(res.to_string())
}

#[get("/divide/{a}/{b}")]
async fn divide(info: web::Path<(i32, i32)>) -> impl Responder {
    let res = simple_actix::divide(info.0, info.1);
    HttpResponse::Ok().body(res.to_string())
}


#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(index)
        .service(add)
        .service(subtract)
        .service(multiply)
        .service(divide)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}