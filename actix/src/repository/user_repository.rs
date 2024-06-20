use mongodb::{ Database, bson::doc };
use crate::model::user::User;

pub async fn insert_user(db: &Database, user: User) -> Result<(), Box<dyn std::error::Error>> {
    let collection = db.collection("user");
    let doc =
        doc! {
        "id": &user.id,
        "first_name": &user.first_name,
        "last_name": &user.last_name,
        "email": &user.email,
        "passwordhashed": &user.password_hased
    };

    collection.insert_one(doc, None).await?;
    Ok(())
}
