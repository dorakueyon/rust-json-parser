use std::collections::BTreeMap;

mod lexer;
mod parser;

use lexer::Lexer;
use parser::{Parser, ParserError};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    String(String),
    Number(f64),
    Boolean(bool),
    Object(BTreeMap<String, Value>),
    Array(Vec<Value>),
}

pub fn parse(input: &str) -> Result<Value, ParserError> {
    let tokens = Lexer::new(input).tokenize();
    return Parser::new(tokens).parse();
}
