use actix_web::{ web, post, HttpResponse, Responder };
use mongodb::Database;
use crate::model::user::User;
use crate::repository::user_repository;

#[post("/user/create")]
pub async fn create_user(user: web::Json<User>, db: web::Data<Database>) -> impl Responder {
    let user = user.into_inner();
    match user_repository::insert_user(&db, user).await {
        Ok(_) => HttpResponse::Ok().body("User created successfully"),
        Err(e) => match e.to_string().as_str() {
            "Email already exists" => HttpResponse::BadRequest().body("Email already exists"),
            _ => HttpResponse::InternalServerError().body(format!("Failed to create user: {}", e)),
        },
    }
}
