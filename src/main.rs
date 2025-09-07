use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use sqlx::PgPool;
mod home;
use home::home_page;
mod blog;
use blog::blog_page;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    println!("Web Server starting!");
    HttpServer::new(|| {
        App::new()
            .service(home_page)
            .service(blog_page)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
