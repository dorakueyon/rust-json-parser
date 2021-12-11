pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
        };
        return lexer;
    }

    pub fn next_token(&mut self) -> Token {
        let token = Token {
            token_type: TokenType::Rbrace,
            literal: "{".to_string(),
        };
        return token;
    }
}

pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

#[derive(PartialEq, Debug)]
pub enum TokenType {
    Lbrace, // {
    Rbrace, // }
    Key,
    Value,
    Colon, // :
}

fn _main() {
    let input = "{test: 'hoge'}".to_string();
    let lexer = Lexer::new(input);
    println!("{:?}", lexer.input);
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
       {test: 'hoge'}
        "#
        .to_string();

        let tests = vec![
            (TokenType::Lbrace, "{".to_string()),
            (TokenType::Key, "test".to_string()),
            (TokenType::Colon, ":".to_string()),
            (TokenType::Value, "hoge".to_string()),
            (TokenType::Rbrace, "}".to_string()),
        ];
        let mut l = Lexer::new(input);
        let mut succeed = true;
        for (i, tt) in tests.iter().enumerate() {
            let tok = l.next_token();
            if tok.token_type != tt.0 {
                println!(
                    "{}: Type match failed. expected: {:?}, got: {:?}",
                    i + 1,
                    &tt.0,
                    &tok.token_type
                );
                succeed = false;
            }
        }
        assert!(succeed);
    }
}
