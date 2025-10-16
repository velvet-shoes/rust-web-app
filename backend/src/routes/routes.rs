use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie},
     post, web, HttpResponse, Responder,
};
use argon2::{PasswordHash, PasswordVerifier};
use diesel::prelude::*;
use crate::{
     models::models::{LoginDto, NewUser, User, RegisterDto}, 
     db::{
          schema::users, 
          db_u::DbPoolUsers
     },
     auth::{
          token::get_jwt, 
          utils::validation_of_credentials,},
};


#[post("/register")]
pub async fn register(
     user_pool: web::Data<DbPoolUsers>,
     user_data: web::Json<RegisterDto>
) ->  impl Responder{
     let check = validation_of_credentials(user_pool.get().expect("fail to conn"), user_data.username.clone()).await;
     match check {
         Ok(_) => {},
         Err(msg) => return HttpResponse::BadRequest().json(
               serde_json::json!({"error": msg}))
     }
     let newuser: NewUser = user_data.into_inner().into();
     let result = web::block(move || {
          let mut conn = user_pool.get().expect("failed to recieve conn from pool");
          diesel::insert_into(users::table)
               .values(newuser)
               .execute(&mut conn)
               .expect("fail to create user in db");
     })
     .await;

     match result {
         Ok(_) => HttpResponse::Ok().json("User created successfully"),
         Err(err) => {
               eprint!("User creation failure: {:?}", err);
               HttpResponse::InternalServerError().finish()
          }
     }
}

#[post("/login")]
pub async fn login(
     user_pool: web::Data<DbPoolUsers>,
     user_data: web::Json<LoginDto>
) -> impl Responder{
     
     let mut conn = user_pool.get().expect("fail to take conn from user pool");
     let username = user_data.username.clone();
     let password = user_data.password.clone();
     let user_c = web::block(move || {
          diesel::QueryDsl::filter(users::table, users::username.eq(username))
               .first::<User>(&mut conn)
               .optional()
               }).await.expect("No such user in db");
     
     let clr_user = match user_c {
          Ok(Some(user)) => user,
          Ok(None) => return HttpResponse::BadRequest().json(serde_json::json!({"erorr": "user not found"})),
          Err(e) => return HttpResponse::BadRequest().body(e.to_string())
     };

     let parsed_pas_hash = PasswordHash::new(&clr_user.passhash)
    .expect("invalid hash in db");
     if argon2::Argon2::default()
          .verify_password(password.as_bytes(), &parsed_pas_hash)
          .is_ok()
          {
               let token = get_jwt(clr_user).await.unwrap();
               let cookie = Cookie::build("token", token.to_owned())
                    .max_age(ActixWebDuration::new(60 * 60, 0))
                    .http_only(true)
                    .finish();
               return HttpResponse::Ok()
                    .cookie(cookie)
                    .json(serde_json::json!({"status": "success", "token": token}))
          }
          

     HttpResponse::Unauthorized().json(serde_json::json!({"erorr": "invalid creds"}))
}



