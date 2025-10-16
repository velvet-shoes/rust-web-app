use diesel::prelude::{AsChangeset, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use crate::db::schema_note::notes;

// --struct for reading from db-- //
#[derive(Serialize, Queryable, Selectable)]
pub struct Note {
     pub id: Option<i32>,
     pub title: String,
     pub creator: String,
     pub text: String,
     pub content: Option<Vec<u8>>,
     pub date_of_creation: Option<String>,
}

#[derive(Deserialize)]
pub struct NoteDTO{
     pub title: String,
     pub text: String,
     pub content: Option<Vec<u8>>,
}

#[derive(Insertable)]
#[diesel(table_name = notes)]
pub struct NewNote {
     pub title: String,
     pub creator: String,
     pub text: String,
     pub content: Option<Vec<u8>>,
}

#[derive(Deserialize)]
pub struct DelNoteDTO{
     pub title: String,
}


#[derive(Deserialize, AsChangeset)]
#[diesel(table_name = notes)]
pub struct UpdNoteDTO{
     pub title: String,
     pub creator: String,
     pub text: String,
     pub content: Option<Vec<u8>>,
}