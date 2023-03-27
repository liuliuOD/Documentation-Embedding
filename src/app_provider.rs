use actix_web::web;

use crate::routes::{v1};

pub fn router(cfg: &mut web::ServiceConfig) {
  // domain includes: /v1/
  cfg.service(
    web::scope("/v1")
      .service(
        web::resource("")
          .route(web::get().to(v1::hello))
      )
      .service(
        web::scope("/echo")
          .service(
            web::resource("")
              .route(web::post().to(v1::echo))
          )
      )
      .service(
        web::scope("/hey")
          .service(
            web::resource("")
              .route(web::get().to(v1::manual_hello))
          )
      ),
  );
}
