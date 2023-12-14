use crate::error::ApiError;
use crate::schema::*;
use actix_web::web;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;
use serde::{Deserialize, Serialize};

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub struct Repository {
    pool: DbPool,
}

// Insertableの宣言でDBにインサートできるようになる
#[derive(Deserialize, Insertable)]
#[diesel(table_name = "posts")]
pub struct NewPost {
    title: String,
    body: String,
}

// Queryableの宣言でDBから取得できるようになる
#[derive(Queryable, Serialize)]
pub struct Post {
    id: i32,
    title: String,
    body: String,
    published: bool,
}

impl Repository {
    pub fn new(database_url: &str) -> Self {
        let manager = ConnectionManager::<SqliteConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("failed to create pool");
        Self { pool }
    }

    pub async fn create_post(&self, new_post: NewPost) -> Result<Post, ApiError> {
        use crate::schema::posts::dsl::*;
        let conn = self.pool.get()?;
        let post = web::block(move || {
            diesel::insert_into(posts::table)
                .values(&new_post)
                .get_result(&mut conn)
        })
        .await??;
        Ok(post)
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
