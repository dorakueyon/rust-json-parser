#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    WhiteSpace,
    String(String),
    Number(f64),
    Boolean(bool),
    Null,

    Lbrace,   // {
    Rbrace,   // }
    Lbracket, //[
    Rbracket, //]
    Colon,    // :
    Comma,
    Illegal(char),
    Eof,
}

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
                '[' => Token::Lbracket,
                ']' => Token::Rbracket,
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
                // null
                'n' => {
                    // TODO: read系はResultにする
                    let result = self.read_null();
                    skip_next = true;
                    if result {
                        Token::Null
                    } else {
                        Token::Illegal(c)
                    }
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

    fn read_number(&mut self) -> f64 {
        let mut number_string = String::new();
        while let Some(&c) = self.chars.peek() {
            if c.is_numeric() || c == '.' {
                self.chars.next();
                number_string.push(c);
            } else {
                break;
            }
        }
        return number_string.parse::<f64>().unwrap();
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

    fn read_null(&mut self) -> bool {
        let mut null_string = String::new();
        for _ in 0..4 {
            null_string.push(*self.chars.peek().unwrap());
            self.chars.next();
        }
        if null_string == "null" {
            return true;
        } else {
            return false;
        }
    }
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
            "float": 25.5,
	        "array": ["name", 321],
            "null": null,
        }
        "#;

        let tests: Vec<Token> = vec![
            Token::Lbrace,
            // begin "number": 123,
            Token::String("number".to_string()),
            Token::Colon,
            Token::Number(123.),
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
            //end

            // begin: "float": 25.5,
            Token::String("float".to_string()),
            Token::Colon,
            Token::Number(25.5),
            Token::Comma,
            //end

            // begin "array": ["name", 321],
            Token::String("array".to_string()),
            Token::Colon,
            Token::Lbracket,
            Token::String("name".to_string()),
            Token::Comma,
            Token::Number(321.),
            Token::Rbracket,
            Token::Comma,
            // end

            // begin "null": null,
            Token::String("null".to_string()),
            Token::Colon,
            Token::Null,
            Token::Comma,
            //end
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
}
