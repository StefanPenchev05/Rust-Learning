use web_server::Server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let server = Server::new(
        String::from("127.0.0.1"),
        8080
    ).await;

    server.run().await
}
