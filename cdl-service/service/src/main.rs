use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, middleware, Responder, web};
use log::{error, info};
use serde_derive::{Deserialize, Serialize};

use cdl_core::lexer;
use cdl_core::lexer::Token;
use cdl_core::parser;
use cdl_core::parser::{Parser};

#[derive(Debug, Serialize, Deserialize)]
struct Request {
  cdl: String
}

#[derive(Debug, Serialize, Deserialize)]
struct PrintRequest {
  ast: Parser
}

#[derive(Debug, Serialize, Deserialize)]
struct PrintResponse {
  cdl: String,
  stats: PrintResponseStats,
}

#[derive(Debug, Serialize, Deserialize)]
struct PrintResponseStats {
  print_time: f64
}

#[derive(Debug, Serialize, Deserialize)]
struct LexResponseStats {
  lex_time: f64,
  lex_mb_sec: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct ParseResponseStats {
  lex_time: f64,
  lex_mb_sec: f64,
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
  ast: Parser,
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
  let cdl_len = item.cdl.len() as f64;
  let bytes_per_second = (cdl_len / lex_time ) * 1000.0;
  info!("Time taken to lex : {} milliseconds", lex_time);
  info!("Bytes per second lexed : {} ", bytes_per_second);
  return match res {
    Ok(tokens) => HttpResponse::Ok().json(LexResponse {
      tokens,
      stats: LexResponseStats {
        lex_time,
        lex_mb_sec : bytes_per_second
      },
    }),
    Err(e) => {
      error!("{:?}", e);
      HttpResponse::InternalServerError().finish()
    }
  };
}

fn _print(_item: web::Json<PrintRequest>) -> HttpResponse {
  let start = std::time::Instant::now();

//  let parser = item.ast;
//  let parser = parser;
//  let ast = parser_to_ast(parser);
//  let res = print_ast(&ast);
//
  let end = start.elapsed();
  let print_time = (end.as_nanos() as f64) / (1000.0 * 1000.0);
  info!("Time taken to parse : {} milliseconds", print_time);
  return HttpResponse::Ok().json(PrintResponse {
    cdl: "".to_string(),
    stats: PrintResponseStats {
      print_time
    },
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

  let cdl_len = item.cdl.len() as f64;
  let bytes_per_second = (cdl_len / lex_time ) * 1000.0;


  info!("Time taken to lex + parse : {} milliseconds", total_time);
  match res {
    Ok(()) => return HttpResponse::Ok().json(ParseResponse {
      stats: ParseResponseStats {
        total_time,
        lex_time,
        parse_time,
        lex_mb_sec : bytes_per_second,
      },
      ast: parser,
    }),
    Err(e) => return HttpResponse::BadRequest().body(e),
  }
}

fn main() -> std::io::Result<()> {
  std::env::set_var("RUST_LOG", "actix_web=debug,service");
  env_logger::init();

  HttpServer::new(||
    App::new()
      .wrap(middleware::Logger::default())
      .wrap(Cors::new())
      .data(web::JsonConfig::default().limit(1024 * 400)) // <- limit size of the payload (global configuration)
      .service(web::resource("/lex").route(web::post().to(lex)))
      .service(web::resource("/parse").route(web::post().to(parse)))
      .service(web::resource("/{name}/{id}/index.html").to(index))
  )
    .bind("127.0.0.1:8081")?
    .run()
}
