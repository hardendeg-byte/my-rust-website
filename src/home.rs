use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
#[get("/")]
pub async fn home_page() -> impl Responder {
    HttpResponse::Ok().body(
        r#"
        <h1>Home Page</h1>
        <p>Click the button to go to the blog page.</p>
        <button onclick="window.location.href='/blog'" style="
            background-color: #c55432ff;
            color: blue;
            padding: 15px 25px;
            text-align: center;
            text-decoration: none;
            display: inline-block;
            border: none;
            cursor: pointer;
            border-radius: 8px;
        ">Go to blog</button>
        "#
    )
}
#TODO: put the css in a separate file