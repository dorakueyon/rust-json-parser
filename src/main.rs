mod lexer;
mod parser;

use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Number(f64),
    Object(BTreeMap<String, Value>),
}

fn _main() {
    let input = "{test: 'hoge'}";
    let _ = lexer::Lexer::new(input);
}

fn main() {
    _main()
}
