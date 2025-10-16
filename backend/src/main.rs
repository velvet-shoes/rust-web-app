use actix_cors::Cors;

use actix_web::{
    HttpServer,
    App,
    web
};
use dotenvy::dotenv;

use crate::{
    db::{
        db_n::init_notes_pool,
        db_u::init_users_pool,
    },
    auth::{
        middle_ware::MiddleWareJwT
    }
};

mod routes;
mod auth;
mod db;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    let sec_key = std::env::var("SECRET_KEY").expect("SECRET_KEY must be set").to_string();
    
    // --USER DB-- //
    let user_db_url = std::env::var("USER_DATABASE_URL").expect("DATABASE_URL must be set");
    let user_pool = init_users_pool(&user_db_url);

    // --NOTE DB-- //
    let notes_db_url = std::env::var("NOTE_DATABASE_URL").expect("DATABASE_URL must be set");
    let notes_pool = init_notes_pool(&notes_db_url);
    println!("{}", "\n 📡 server started on 127.0.0.1:8080 📡");


    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:5173")
                    .allowed_origin("http://127/0.0.1:5173")
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
                    .allowed_headers(vec!["Content-Type", "Authorization"])
                    .supports_credentials()
            )

            .app_data(web::Data::new(user_pool.clone()))
            .app_data(web::Data::new(notes_pool.clone()))
            
            .service(web::scope("/api")
                .service(
                    web::scope("/notes")
                        .wrap(MiddleWareJwT::new(sec_key.clone()))
                        .service(routes::notes_routes::create_note)
                        // .service(routes::notes_routes::read_note) // not implemented in frontend
                        .service(routes::notes_routes::update_note)
                        .service(routes::notes_routes::delete_note)
                        .service(routes::notes_routes::get_all_notes)
                )
                .service(
                    web::scope("/auth")
                        .service(routes::routes::register)
                        .service(routes::routes::login)
                )
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

