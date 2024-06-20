use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder{
    HttpResponse::Ok().body("Hello from the server!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder{
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hello", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
