use mongodb::{ Database, bson::doc };
use crate::model::user::User;

pub async fn insert_user(db: &Database, user: User) -> Result<(), Box<dyn std::error::Error>> {
    let collection = db.collection("user");
    let doc =
        doc! {
        "first_name": &user.first_name,
        "last_name": &user.last_name,
        "email": &user.email,
        "passwordhashed": &user.password_hased
    };

    match collection
        .find_one(doc! {"email": &user.email}, None)
        .await
    {
        Ok(Some(_)) => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Email already exists"))),
        Ok(None) => (),
        Err(e) => return Err(Box::new(e)),
    }
    collection.insert_one(doc, None).await?;
    Ok(())
}
