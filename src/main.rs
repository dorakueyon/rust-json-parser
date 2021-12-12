use core::num;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct LexerError {
    pub msg: String,
}

impl LexerError {
    pub fn new(msg: &str) -> Self {
        LexerError {
            msg: msg.to_string(),
        }
    }
}

pub struct Lexer<'a> {
    //input: String,
    chars: std::iter::Peekable<std::str::Chars<'a>>,
}

impl<'a> Lexer<'a> {
    /// 文字列を受け取りLexerを返す
    pub fn new(input: &str) -> Lexer {
        Lexer {
            chars: input.chars().peekable(),
        }
        //return lexer;
    }

    fn skip_whitespace(&mut self) {
        loop {
            let peeked = self.chars.peek();
            if peeked.is_none() {
                break;
            }
            let c = peeked.unwrap();
            if *c == ' ' || *c == '\t' || *c == '\n' || *c == '\r' {
                self.chars.next();
            } else {
                break;
            }
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = vec![];
        while let Some(token) = self.next_token() {
            match token {
                Token::WhiteSpace => {}
                _ => tokens.push(token),
            }
        }
        return tokens;
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        let mut skip_next = false;
        let token = match self.chars.peek() {
            Some(&c) => match c {
                //c if c.is_whitespace() || c == '\n' => self.new_token(TokenType::WhiteSpace),
                '{' => Token::Lbrace,
                '}' => Token::Rbrace,
                ':' => Token::Colon,
                ',' => Token::Comma,
                // string
                '"' => {
                    self.chars.next();
                    let s = self.read_string();
                    skip_next = true;
                    Token::String(s)
                }
                // number
                c if c.is_numeric() => {
                    let n = self.read_number();
                    skip_next = true;
                    Token::Number(n)
                }
                // boolean: true
                't' => {
                    let bool = self.read_bool();
                    skip_next = true;
                    Token::Boolean(bool)
                }
                // boolean: false
                'f' => {
                    let bool = self.read_bool();
                    skip_next = true;
                    Token::Boolean(bool)
                }
                _ => Token::Illegal(c.clone()),
            },
            None => return None,
        };

        if !skip_next {
            self.chars.next();
        }
        return Some(token);
    }
    fn read_string(&mut self) -> String {
        let mut result = String::new();
        while let Some(c) = self.chars.next() {
            match c {
                '\"' => break,
                _ => result.push(c),
            }
        }
        return result;
    }

    fn read_number(&mut self) -> u64 {
        let mut number_string = String::new();
        while let Some(&c) = self.chars.peek() {
            if c.is_numeric() {
                self.chars.next();
                number_string.push(c);
            } else {
                break;
            }
        }
        return number_string.parse::<u64>().unwrap();
    }

    fn read_bool(&mut self) -> bool {
        let mut bool_string = String::new();
        for _ in 0..4 {
            bool_string.push(*self.chars.peek().unwrap());
            self.chars.next();
        }
        if bool_string == "true" {
            return true;
        } else {
            return false;
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    WhiteSpace,
    String(String),
    Number(u64),
    Boolean(bool),

    Lbrace, // {
    Rbrace, // }
    Colon,  // :
    Comma,
    Illegal(char),
    Eof,
}

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

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Object(BTreeMap<String, Value>),
}

fn _main() {
    let input = "{test: 'hoge'}";
    let lexer = Lexer::new(input);
    println!("{:?}", lexer.chars);
}

fn main() {
    _main()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_next_token() {
        let input = r#"
        {
            "number": 123,
            "boolean": true,
            "string": "hoge",
        }
        "#;

        let tests: Vec<Token> = vec![
            Token::Lbrace,
            // begin "number": 123,
            Token::String("number".to_string()),
            Token::Colon,
            Token::Number(123),
            Token::Comma,
            //// end

            // begin: "booealn": true,
            Token::String("boolean".to_string()),
            Token::Colon,
            Token::Boolean(true),
            Token::Comma,
            ////end

            // begin: "string": "hoge",
            Token::String("string".to_string()),
            Token::Colon,
            Token::String("hoge".to_string()),
            Token::Comma,
            Token::Rbrace,
        ];
        let mut l = Lexer::new(input);
        let mut succeed = true;
        for (i, tt) in tests.iter().enumerate() {
            match l.next_token() {
                Some(tok) => {
                    if tok != *tt {
                        println!(
                            "{}: Type match failed. expected: {:?}, got: {:?}",
                            i + 1,
                            &tt,
                            &tok
                        );
                        succeed = false;
                    }
                }
                None => {
                    println!("no token found");
                    succeed = false;
                }
            }
        }
        assert!(succeed);
    }

    #[test]
    fn test_parse() {
        let input = r#"
        {
            "key": "value"
        }
        "#;
        let l = Lexer::new(input).tokenize();
        let mut p = Parser::new(l);
        let value = p.parse().unwrap();
        let mut object = BTreeMap::new();
        object.insert("key".to_string(), Value::String("value".to_string()));
        assert_eq!(value, Value::Object(object));
    }
}
