use diesel::prelude::*;
use actix_web::{
     get, post, put, delete, web, HttpMessage, HttpRequest, HttpResponse, Responder
};

use crate::{
     db::db_n::PoolNotes, 
     models::{
          model_notes::{DelNoteDTO, NewNote, Note, NoteDTO, UpdNoteDTO}, 
          models::Claims,
     }, 
     db::schema_note::notes
};

#[post("/")]
pub async fn create_note(
     db_pool: web::Data<PoolNotes>,
     note_data: web::Json<NoteDTO>,
     req: HttpRequest
) -> impl Responder {
     let extens = req.extensions();
     let claims = extens.get::<Claims>().expect("fail to get claims from req"); 
          
     let newnote = NewNote{
          title: note_data.title.clone(),
          creator: claims.sub.clone(),
          text: note_data.text.clone(),
          content: note_data.content.clone()
     };

     let result = web::block(move || {
          let mut conn = db_pool.db_pool_notes.get().expect("fail to recieve conn from pool");
          diesel::insert_into(notes::table)
               .values(newnote)
               .execute(&mut conn)
               .expect("fail to create note in db");
     }).await;

     match result {
          Ok(_) => HttpResponse::Ok().json("Note created successfuly"),
          Err(err) => {
               eprint!("Note creation failure: {:?}", err);
               HttpResponse::InternalServerError().finish()
          }
     }
}

#[get("/{title}")]
pub async fn read_note(
     db_pool: web::Data<PoolNotes>,
     title_read: String) 
-> impl Responder {
          let result = web::block(move || {
               let mut conn = db_pool.db_pool_notes.get().expect("fail to recieve conn from pool");

               diesel::QueryDsl::filter(notes::table, notes::title.eq(title_read))
                    .first::<Note>(&mut conn)
                    .optional()

          }).await.expect("No such note in db");

          match result {
          Ok(Some(note)) => return HttpResponse::Ok().json(note),
          Ok(None) => return HttpResponse::BadRequest().json(serde_json::json!({"erorr": "note not found"})),
          Err(e) => return HttpResponse::BadRequest().body(e.to_string())
          }  
}

#[delete("/")]
pub async fn delete_note(
     db_pool: web::Data<PoolNotes>,
     note_data: web::Json<DelNoteDTO>,
) -> impl Responder {



     let result = web::block(move || {
          let mut conn = db_pool.db_pool_notes.get().expect("fail to recieve conn from pool");
          diesel::delete(notes::table.filter(notes::title.eq(note_data.title.clone())))
               .execute(&mut conn)
               .expect("fail to delete note from db");
     }).await;

     match result {
          Ok(_) => HttpResponse::Ok().json("Note deleted successfuly"),
          Err(err) => {
               eprint!("Note creation failure: {:?}", err);
               HttpResponse::InternalServerError().finish()
          }
     }
}

#[put("/")]
pub async fn update_note(
     db_pool: web::Data<PoolNotes>,
     upd_data: web::Json<NoteDTO>,
     req: HttpRequest
) -> impl Responder {

     let extens = req.extensions();
     let claims = extens.get::<Claims>().expect("fail to get claims from req"); 
          
     let upd_note = UpdNoteDTO{
          title: upd_data.title.clone(),
          creator: claims.sub.clone(),
          text: upd_data.text.clone(),
          content: upd_data.content.clone()
     };

     let result = web::block(move || {
          let mut conn = db_pool.db_pool_notes.get().expect("fail to recieve conn from pool");
          diesel::update(notes::table.filter(notes::title.eq(upd_data.title.clone())))
               .set(upd_note)
               .execute(&mut conn)
               .expect("fail to update note")
     }).await;

     match result {
          Ok(_) => HttpResponse::Ok().json("Note updated successfuly"),
          Err(err) => {
               eprint!("Note creation failure: {:?}", err);
               HttpResponse::InternalServerError().finish()
          }
     }
}

#[get("/")]
pub async fn get_all_notes(db_pool: web::Data<PoolNotes>) -> impl Responder{
     use crate::db::schema_note::notes::dsl::notes;

     let result = web::block(move || {
          let mut conn = db_pool.db_pool_notes.get().expect("fail to recieve conn from pool");
          notes.load::<Note>(&mut conn);
     }).await;

    match result {
        Ok(note_list) => HttpResponse::Ok().json(note_list),
        Err(blocking_err) => {
            eprintln!("error: {:?}", blocking_err);
            HttpResponse::InternalServerError().finish()
        }
    }
}