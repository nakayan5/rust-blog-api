mod error;
mod repository;
mod schema;

use actix_web::{web, App, HttpResponse, HttpServer};
use error::ApiError;
use repository::{NewPost, Repository};

#[actix_web::post("/posts")]
async fn create_post(
    repo: web::Data<Repository>,
    new_post: web::Json<NewPost>,
) -> Result<HttpResponse, ApiError> {
    let new_post = new_post.into_inner();
    let post = repo.create_post(new_post).await?;
    Ok(HttpResponse::Ok().json(post))
}

#[actix_web::main]
fn main() {
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let repo = web::Data::new(Repository::new(&database_url));

    HttpServer::new(move || {
        App::new().app_data(repo.clone()).service(create_post)
    }).bind("127.0.0.1:8080")?.run().await()
}
