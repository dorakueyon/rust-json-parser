use crate::{lexer::Token, Value};
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct ParserError {
    pub msg: String,
}

impl ParserError {
    pub fn new(msg: &str) -> Self {
        ParserError {
            msg: msg.to_string(),
        }
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let parser = Parser { tokens, index: 0 };
        return parser;
    }

    fn peek_token(&mut self) -> Result<&Token, ParserError> {
        self.tokens
            .get(self.index)
            .ok_or_else(|| ParserError::new("error: a token isn't peekable"))
    }

    pub fn parse(&mut self) -> Result<Value, ParserError> {
        let token = self.peek_token()?.clone();
        let result = match token {
            Token::Lbrace => self.parse_object(),
            Token::String(s) => {
                self.next_expect()?;
                Ok(Value::String(s))
            }
            Token::Number(num) => {
                self.next_expect()?;
                Ok(Value::Number(num))
            }
            _ => Err(ParserError::new(&format!(
                "error: a token must start {{ or [ or string or number or bool or null {:?}",
                token
            ))),
        };
        result
    }

    fn parse_object(&mut self) -> Result<Value, ParserError> {
        let token = self.peek_token()?;
        if *token != Token::Lbrace {
            return Err(ParserError::new(&format!(
                "error: JSON object must starts {{ {:?}",
                token
            )));
        }

        // skip {
        self.next_expect()?;

        let mut object = BTreeMap::new();
        if *self.peek_token()? == Token::Rbrace {
            return Ok(Value::Object(object));
        }

        loop {
            let token1 = self.next_expect()?.clone();
            let token2 = self.next_expect()?;
            match (token1, token2) {
                (Token::String(key), &Token::Colon) => {
                    object.insert(key, self.parse()?);
                }
                _ => {
                    return Err(ParserError::new(
                        "error: a pair (key(string)) and : token) is required",
                    ))
                }
            }
            println!("{:?}", self.tokens);
            println!("object {:?}, index: {}", object, self.index);
            let token3 = self.next_expect()?;
            match token3 {
                Token::Rbrace => return Ok(Value::Object(object)),
                Token::Comma => continue,
                _ => {
                    return Err(ParserError::new(&format!(
                        "error: a {{, or token is expected: {:?}",
                        token3
                    )));
                }
            }
        }
    }

    // get token from front and increment index
    fn next_expect(&mut self) -> Result<&Token, ParserError> {
        self.index += 1;
        self.tokens
            .get(self.index - 1)
            .ok_or_else(|| ParserError::new("error: a token isn't peekable"))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{lexer::Lexer, Value};

    #[test]
    fn test_parse() {
        let input = r#"
        {
            "key": "value",
            "number": 123
        }
        "#;
        let l = Lexer::new(input).tokenize();
        let mut p = Parser::new(l);
        let value = p.parse().unwrap();
        let mut object = BTreeMap::new();
        object.insert("key".to_string(), Value::String("value".to_string()));
        object.insert("number".to_string(), Value::Number(123.));
        assert_eq!(value, Value::Object(object));
    }
}