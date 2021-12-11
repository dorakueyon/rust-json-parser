use core::num;

#[derive(Debug)]
pub struct LexerError {
    pub msg: String,
}

impl LexerError {
    fn new(msg: &str) -> Self {
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

    fn new_token(&self, token_type: TokenType) -> Token {
        Token { token_type }
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

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        let mut skip_next = false;
        let token = match self.chars.peek() {
            Some(&c) => match c {
                //c if c.is_whitespace() || c == '\n' => self.new_token(TokenType::WhiteSpace),
                '{' => self.new_token(TokenType::Lbrace),
                '}' => self.new_token(TokenType::Rbrace),
                ':' => self.new_token(TokenType::Colon),
                ',' => self.new_token(TokenType::Comma),
                // string
                '"' => {
                    self.chars.next();
                    let s = self.read_string();
                    skip_next = true;
                    self.new_token(TokenType::String(s))
                }
                // number
                c if c.is_numeric() => {
                    let n = self.read_number();
                    skip_next = true;
                    self.new_token(TokenType::Number(n))
                }
                // boolean: true
                't' => {
                    let bool = self.read_bool();
                    skip_next = true;
                    self.new_token(TokenType::Boolean(bool))
                }
                // boolean: false
                'f' => {
                    let bool = self.read_bool();
                    skip_next = true;
                    self.new_token(TokenType::Boolean(bool))
                }
                _ => self.new_token(TokenType::Illegal(c.clone())),
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

pub struct Token {
    pub token_type: TokenType,
}

pub enum Boolean {}

#[derive(PartialEq, Debug)]
pub enum TokenType {
    WhiteSpace,
    String(String),
    Number(u64),
    Boolean(bool),

    Lbrace, // {
    Rbrace, // }
    Colon,  // :
    Comma,
    Illegal(char),
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

        let tests: Vec<TokenType> = vec![
            TokenType::Lbrace,
            // begin "number": 123,
            TokenType::String("number".to_string()),
            TokenType::Colon,
            TokenType::Number(123),
            TokenType::Comma,
            //// end

            // begin: "booealn": true,
            TokenType::String("boolean".to_string()),
            TokenType::Colon,
            TokenType::Boolean(true),
            TokenType::Comma,
            ////end

            // begin: "string": "hoge",
            TokenType::String("string".to_string()),
            TokenType::Colon,
            TokenType::String("hoge".to_string()),
            TokenType::Comma,
            TokenType::Rbrace,
        ];
        let mut l = Lexer::new(input);
        let mut succeed = true;
        for (i, tt) in tests.iter().enumerate() {
            match l.next_token() {
                Some(tok) => {
                    if tok.token_type != *tt {
                        println!(
                            "{}: Type match failed. expected: {:?}, got: {:?}",
                            i + 1,
                            &tt,
                            &tok.token_type
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
}
