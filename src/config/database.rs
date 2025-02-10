use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

use super::settings::Settings;

pub type DBPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> DBPool {
    let config = Settings::new();

    let manager = ConnectionManager::<PgConnection>::new(config.database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to connect to database!")
}
