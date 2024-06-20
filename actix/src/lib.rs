use actix_web::{ middleware, web, App, HttpServer };
use mongodb::{ Client, options::ClientOptions };
use std::env;
use dotenv::dotenv;

mod model;
mod api;
mod repository;

use api::user;

pub struct Server {
    db: mongodb::Database,
    port: u16,
    ip_v4: String,
}

impl Server {
    pub async fn new(ip_v4: String, port: u16) -> Self {
        dotenv().ok();
        let client_options = env::var("CLIENT_OPTIONS").expect("CLIENT_OPTIONS must be set");
        let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");
        let db = Server::get_database(client_options, database_name).await;
        Self { db, port, ip_v4 }
    }

    async fn get_database(client_options: String, database_name: String) -> mongodb::Database {
        let client_options = ClientOptions::parse(&client_options).await.unwrap();
        let client = Client::with_options(client_options).unwrap();
        let db = client.database(&database_name);
        db
    }

    pub async fn run(&self) -> std::io::Result<()> {
        let db = self.db.clone();
        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(db.clone()))
                .service(user::create_user)
                .wrap(middleware::Logger::default())
        })
        .bind((self.ip_v4.clone(), self.port))?
        .run()
        .await
    }
}
