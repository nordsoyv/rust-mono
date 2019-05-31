#![allow(unused_imports)]

mod cdl;
mod common;
mod lexer;
mod xmlparser;

fn main() {
  println!("Hello, world!");
  let s = "This is a string";
  dbg!(s);
  let slice = &s[4..9];
  dbg!(slice);
}
