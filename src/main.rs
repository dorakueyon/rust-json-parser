use core::num;
use std::collections::BTreeMap;

#[derive(Debug)]
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

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let parser = Parser { tokens };
        return parser;
    }

    pub fn parse(&mut self) -> Value {
        unimplemented!()
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
            "key": "value",
        }
        "#;
        let l = Lexer::new(input).tokenize();
        let mut p = Parser::new(l);
        let value = p.parse();
        let mut object = BTreeMap::new();
        object.insert("key".to_string(), Value::String("value".to_string()));
        assert_eq!(value, Value::Object(object));
    }
}
