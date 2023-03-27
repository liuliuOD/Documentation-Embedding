use actix_web::{App, HttpServer};

use Documentation_Embedding::app_provider::router;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
      .configure(router)
  })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
