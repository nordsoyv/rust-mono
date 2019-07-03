use actix_web::{App, HttpServer, Responder, web, middleware, HttpResponse};
use serde_derive::{Deserialize, Serialize};
use cdl_core::lexer;

#[derive(Debug, Serialize, Deserialize)]
struct LexRequest {
  cdl: String
}

fn index(info: web::Path<(String, u32)>) -> impl Responder {
  format!("Hello {}! id:{}", info.0, info.1)
}

fn lex(item: web::Json<LexRequest>) -> HttpResponse {
  let lexer = lexer::Lexer::new();
  let res = lexer.lex(item.cdl.clone());
  return match res {
    Ok(tokens) => HttpResponse::Ok().json(tokens),
    Err(e) => {
      println!("{:?}", e);
      HttpResponse::InternalServerError().finish()
    }
  };
}

fn main() -> std::io::Result<()> {
  std::env::set_var("RUST_LOG", "actix_web=info");
  env_logger::init();

  HttpServer::new(||
    App::new()
      .wrap(middleware::Logger::default())
      .data(web::JsonConfig::default().limit(1024 * 100)) // <- limit size of the payload (global configuration)
      .service(web::resource("/lex").route(web::post().to(lex)))
      .service(web::resource("/{name}/{id}/index.html").to(index))
  )
    .bind("127.0.0.1:8080")?
    .run()
}
