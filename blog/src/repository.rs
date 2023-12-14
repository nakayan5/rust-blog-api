use crate::schema::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub struct Repository {
    pool: DbPool,
}

impl Repository {
    pub fn new(database_url: &str) -> Self {
        let manager = ConnectionManager::<SqliteConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("failed to create pool");
        Self { pool }
    }
}

#[cfg(test)]
mod tests {
    use actix_web::web::Data;

    use super::*;

    #[test]
    fn test_conn() {
        let database_url = std::env::var("DATABASE_URL").unwrap();
        let repo = Repository::new(&database_url);
        assert!(repo.pool.get().is_ok());
    }
}
