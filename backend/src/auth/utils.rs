use argon2::{
     password_hash::{
          SaltString, rand_core::OsRng
     }, Argon2, PasswordHasher
};
use diesel::{
     prelude::*,
     dsl::{exists, select},
     r2d2::{ConnectionManager}
};

use crate::{
     models::models::{NewUser, RegisterDto},
      db::schema::users::{self, username}};

impl From<RegisterDto> for NewUser {
     fn from(dto: RegisterDto) -> Self {
         let salt= SaltString::generate(&mut OsRng);
         let argon2 = Argon2::default();
         let passhash = argon2
               .hash_password(dto.password.as_bytes(), &salt)
               .expect("fail to hash password")
               .to_string();
          NewUser { 
               username: dto.username, 
               passhash: passhash, 
               email: dto.email 
          }
     }
}

pub async fn validation_of_credentials(
     mut conn: diesel::r2d2::PooledConnection<ConnectionManager<SqliteConnection>>,
     user_name: String,
) -> Result<(), String>{
     // let mut conn = user_pool.get().expect("failed to recieve conn from pool");
     let if_user_exist  = select(exists(users::table.filter(username.eq(user_name))))
          .get_result::<bool>(&mut conn)
          .expect("fail of user checking");   

     if if_user_exist {
          Err(format!("user already exist"))
     } else {
          Ok(())
     }
}

