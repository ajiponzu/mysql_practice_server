mod queries;

use actix_web::{get, web, HttpResponse, Responder};

#[get("/")]
pub async fn hello() -> impl Responder {
  HttpResponse::Ok().body("Hello mysql-practice-server!")
}

#[get("/q{question_id}")]
pub async fn get_answer(web::Path(question_id): web::Path<String>) -> impl Responder {
  let id: u16 = question_id.parse().unwrap_or(0);
  let mut msg = String::from("bad query...");
  if id != 0 {
    msg = queries::queries(&id)
  }
  HttpResponse::Ok().body(msg)
}

pub async fn manual_hello() -> impl Responder {
  HttpResponse::Ok().body("Hello!")
}
