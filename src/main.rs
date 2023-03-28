use actix_web::{App, HttpServer};

mod routes;
mod app_provider;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
      .configure(app_provider::router)
  })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
