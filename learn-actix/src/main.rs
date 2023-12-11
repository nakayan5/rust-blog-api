use actix_web::{web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};

// #[actix_web::get("/")] // アトリビュートマクロ
// async fn hello() -> HttpResponse {
//     HttpResponse::Ok()
//         .append_header(ContentType::json())
//         .body(r#"{"greet":"Hello, World!"}"#) // r#""#は文字列リテラル記法　エスケープなしに""をかける
// }
// curl "http://localhost:8000/"  -> {"greet":"Hello, World!"}

// Deserializeをderive（導く）する
#[derive(Deserialize)]
struct HelloQuery {
    name: String,
    age: String,
}

// Serializeをderive（導く）する
#[derive(Serialize)]
struct HelloResponse {
    greet: String,
}

#[actix_web::get("/")]
async fn hello(query: web::Query<HelloQuery>) -> HttpResponse {
    let query = query.into_inner();
    let message = format!(
        "Hello, my name is {} I am {} years old.",
        query.name, query.age
    );

    let h = HelloResponse { greet: message };

    HttpResponse::Ok().json(h)
}
// curl "http://localhost:8000?name=Tom&age=20/"  -> {"greet":"Hello, my name is Tom I am 20 years old."}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind(("0.0.0.0", 8000))?
        .run()
        .await
}
// bindはResult<T, E>の値を返す
// これに？を適用すると失敗時はEを成功時はTを返してくれる
