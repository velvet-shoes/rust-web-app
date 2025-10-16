use diesel::{r2d2::{ConnectionManager, Pool}, SqliteConnection};
#[derive(Clone)]
pub struct PoolNotes{
    pub db_pool_notes: Pool<ConnectionManager<SqliteConnection>>
}
pub fn init_notes_pool(database_url: &str) -> PoolNotes {
    let notes_db_manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let notes_db_pool = PoolNotes{
        db_pool_notes: Pool::builder()
                        .build(notes_db_manager)
                        .unwrap()};
    return notes_db_pool;
}