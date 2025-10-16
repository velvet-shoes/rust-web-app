use diesel::prelude::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use crate::db::schema::users;

////     User structs     ////
//read from db
#[derive(Serialize, Deserialize, Queryable)]
pub struct User {
     pub id: Option<i32>,
     pub username: String,
     pub passhash: String,
     pub email: String
}

//registerDTO
#[derive(Deserialize)]
pub struct RegisterDto {
     pub username: String,
     pub password: String,
     pub email: String

}

//loginDTO
#[derive(Deserialize)]
pub struct LoginDto {
     pub username: String,
     pub password: String,
}

//register
#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
     pub username: String,
     pub passhash: String,
     pub email: String
}

////     JWT structs     ////
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    //role: String,
    pub exp: usize,
}




