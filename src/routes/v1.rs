use actix_web::{HttpResponse, Responder};

pub async fn hello() -> impl Responder {
  HttpResponse::Ok().body("Hello world!")
}

pub async fn echo(req_body: String) -> impl Responder {
  HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
  HttpResponse::Ok().body("Hey there!")
}
