mod lexer;
mod parser;

use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    String(String),
    Number(f64),
    Boolean(bool),
    Object(BTreeMap<String, Value>),
    Array(Vec<Value>),
}

fn _main() {
    let input = "{test: 'hoge'}";
    let _ = lexer::Lexer::new(input);
}

fn main() {
    _main()
}
