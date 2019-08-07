use actix_web::{App, HttpResponse, HttpServer, middleware, Responder, web};
use log::{error, info};
use serde_derive::{Deserialize, Serialize};

use cdl_core::lexer;
use cdl_core::lexer::Token;
use cdl_core::parser;
use cdl_core::parser::{Ast, parser_to_ast};
use cdl_core::print::{print_ast};

#[derive(Debug, Serialize, Deserialize)]
struct Request {
  cdl: String
}

#[derive(Debug, Serialize, Deserialize)]
struct PrintRequest {
  ast: Ast
}

#[derive(Debug, Serialize, Deserialize)]
struct LexResponseStats {
  lex_time: f64
}

#[derive(Debug, Serialize, Deserialize)]
struct ParseResponseStats {
  lex_time: f64,
  parse_time: f64,
  total_time: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct LexResponse {
  stats: LexResponseStats,
  tokens: Vec<Token>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ParseResponse {
  stats: ParseResponseStats,
  ast: Ast,
}

#[derive(Debug, Serialize, Deserialize)]
struct PrintResponse {
  cdl: String,
}


fn index(info: web::Path<(String, u32)>) -> impl Responder {
  format!("Hello {}! id:{}", info.0, info.1)
}

fn lex(item: web::Json<Request>) -> HttpResponse {
  let start = std::time::Instant::now();
  let lexer = lexer::Lexer::new();
  let res = lexer.lex(item.cdl.clone());
  let end = start.elapsed();
  let lex_time = (end.as_nanos() as f64) / (1000.0 * 1000.0);
  info!("Time taken to lex : {} milliseconds", lex_time);
  return match res {
    Ok(tokens) => HttpResponse::Ok().json(LexResponse {
      tokens,
      stats: LexResponseStats {
        lex_time: lex_time
      },
    }),
    Err(e) => {
      error!("{:?}", e);
      HttpResponse::InternalServerError().finish()
    }
  };
}

fn print(item: web::Json<PrintRequest>)-> HttpResponse {
  let ast = &item.ast;
  let cdl = print_ast(ast);
  return HttpResponse::Ok().json(PrintResponse {
    cdl
  });
}


fn parse(item: web::Json<Request>) -> HttpResponse {
  let start_lex = std::time::Instant::now();
  let lexer = lexer::Lexer::new();
  let res = lexer.lex(item.cdl.clone());
  let end_lex = start_lex.elapsed();

  let tokens = match res {
    Ok(tokens) => tokens,
    Err(e) => {
      error!("{:?}", e);
      return HttpResponse::InternalServerError().finish();
    }
  };
  let start_parse = std::time::Instant::now();
  let mut parser = parser::Parser::new();
  let res = parser.parse(tokens);
  let end_parse = start_parse.elapsed();

  let end_total = start_lex.elapsed();

  let lex_time = (end_lex.as_nanos() as f64) / (1000.0 * 1000.0);
  let parse_time = (end_parse.as_nanos() as f64) / (1000.0 * 1000.0);
  let total_time = (end_total.as_nanos() as f64) / (1000.0 * 1000.0);

  info!("Time taken to lex + parse : {} milliseconds", total_time);
  match res {
    Ok(()) => {
      return HttpResponse::Ok().json(ParseResponse {
        stats: ParseResponseStats {
          total_time,
          lex_time,
          parse_time,
        },
        ast : parser_to_ast(parser),
      });
    }
    Err(e) => return HttpResponse::BadRequest().body(e),
  }
}

fn main() -> std::io::Result<()> {
  std::env::set_var("RUST_LOG", "actix_web=info,service");
  env_logger::init();

  HttpServer::new(||
    App::new()
      .wrap(middleware::Logger::default())
      .data(web::JsonConfig::default().limit(1024 * 500)) // <- limit size of the payload (global configuration)
      .service(web::resource("/lex").route(web::post().to(lex)))
      .service(web::resource("/parse").route(web::post().to(parse)))
      .service(web::resource("/print").route(web::post().to(print)))
      .service(web::resource("/{name}/{id}/index.html").to(index))
  )
    .bind("127.0.0.1:8080")?
    .run()
}
