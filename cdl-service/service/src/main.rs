use actix_web::{App, HttpServer, Responder, web, middleware, HttpResponse};
use serde_derive::{Deserialize, Serialize};
use cdl_core::lexer;
use cdl_core::parser;

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

fn parse(item: web::Json<LexRequest>) -> HttpResponse {
  let lexer = lexer::Lexer::new();
  let res = lexer.lex(item.cdl.clone());

  let tokens = match res {
    Ok(tokens) => tokens,
    Err(e) => {
      println!("{:?}", e);
      return HttpResponse::InternalServerError().finish()
    }
  };

  let mut parser = parser::Parser::new();
  let res = parser.parse(tokens);
  match res {
    Ok(()) =>return HttpResponse::Ok().json(parser),
    Err(e)=> return HttpResponse::BadRequest().body(e),
  }

}

fn main() -> std::io::Result<()> {
  std::env::set_var("RUST_LOG", "actix_web=info");
  env_logger::init();

  HttpServer::new(||
    App::new()
      .wrap(middleware::Logger::default())
      .data(web::JsonConfig::default().limit(1024 * 100)) // <- limit size of the payload (global configuration)
      .service(web::resource("/lex").route(web::post().to(lex)))
      .service(web::resource("/parse").route(web::post().to(parse)))
      .service(web::resource("/{name}/{id}/index.html").to(index))
  )
    .bind("127.0.0.1:8080")?
    .run()
}
