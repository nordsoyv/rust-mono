mod lex;
mod parse;
mod print;
mod select;

use parse::ParseResult;
use parse::Parser;
pub use lex::Lexer;
pub use select::{select_field, select_entity};
use lex::LexItem;

pub fn compile(cdl: String) -> Result<Vec<LexItem>, String> {
    let lexer = Lexer::new(cdl);
    let lex_items = lexer.lex().unwrap();
//    let parser = Parser::new(lex_items);
//    let root = parser.parse();
//    root
    Ok(lex_items)
}

pub fn print(root: ParseResult) -> String {
    print::print(root)
}
