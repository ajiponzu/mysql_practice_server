extern crate mysql_practice_server;

use actix_web::{web, App, HttpServer};
use mysql_practice_server::requests;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
      .service(requests::hello)
      .service(requests::get_answer)
      .route("/route", web::get().to(requests::manual_hello))
  })
  .bind("127.0.0.1:8000")?
  .run()
  .await
}
