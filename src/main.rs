use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use
#[get("/")]
async fn home_page() -> impl Responder {
    HttpResponse::Ok().body(
        r#"
        <h1>Home Page (with JavaScript)</h1>
        <p>Click the button to go to the destination page.</p>
        <button onclick="window.location.href='/info'" style="
            background-color: #4CAF50;
            color: white;
            padding: 15px 25px;
            text-align: center;
            text-decoration: none;
            display: inline-block;
            border: none;
            cursor: pointer;
            border-radius: 8px;
        ">Go to Destination</button>
        "#
    )
}



#[get("/info")]
async fn get_info() -> impl Responder {
    HttpResponse::Ok().body("It is 2 am")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    println!("Web Server starting!");
    HttpServer::new(|| {
        App::new()
            .service(home_page)
            .service(get_info)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
