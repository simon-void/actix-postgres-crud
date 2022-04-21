use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    println!("starting server on port {}", port);
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
    })
        .bind(("127.0.0.1", port))?
        .run()
        .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello Actix".to_lowercase())
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    let res_body = format!("faint echo: {}", req_body.to_lowercase());
    HttpResponse::Ok().body(res_body)
}
