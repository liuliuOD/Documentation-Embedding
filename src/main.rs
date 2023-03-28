use actix_web::{App, HttpServer, middleware};

mod routes;
mod app_provider;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

  HttpServer::new(|| {
    App::new()
      .wrap(middleware::Logger::default())
      .configure(app_provider::router)
  })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
