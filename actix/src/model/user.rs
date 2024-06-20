use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct User{
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password_hased: String
}