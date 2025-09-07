use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
#[get("/blog")]
pub async fn blog_page() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the blog page!")
}