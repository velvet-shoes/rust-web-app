use diesel::r2d2::{Pool, ConnectionManager};
use diesel::sqlite::SqliteConnection;

pub type DbPoolUsers = Pool<ConnectionManager<SqliteConnection>>;

pub fn init_users_pool(database_url: &str) -> DbPoolUsers {
     let user_db_manager = ConnectionManager::<SqliteConnection>::new(database_url);
     let user_db_pool = Pool::builder()
          .build(user_db_manager)
          .unwrap();
     return user_db_pool;
}