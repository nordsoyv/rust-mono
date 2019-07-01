use actix_web::{App, HttpServer, Responder, web};

fn index(info: web::Path<(String, u32)>) -> impl Responder {
  format!("Hello {}! id:{}", info.0, info.1)
}

fn main() -> std::io::Result<()> {
  HttpServer::new(||
    App::new()
      .service(web::resource("/{name}/{id}/index.html").to(index)))
    .bind("127.0.0.1:8080")?
    .run()
}
