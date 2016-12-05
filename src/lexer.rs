use token::Token;
use std::str::Chars;
use std::iter::Peekable;



pub struct Lexer<'a> {
    // input: &'a str,
    // pos: usize,
    // read_position: usize,
    input_iter: Peekable<Chars<'a>>, // ch: char,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer { input_iter: input.chars().peekable() }
    }

    fn read_char(&mut self) -> Option<char> {
        self.input_iter.next()
    }

    fn is_letter(c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }


    fn lookup_keyword(id: String) -> Option<Token> {
        match id.as_str() {
            "let" => Some(Token::Let),
            "fn" => Some(Token::Function),
            _ => Some(Token::Ident(id)),
        }
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.input_iter.peek()
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.peek_char() {
            if c.is_whitespace() {
                let _ = self.read_char();
            } else {
                break;
            }
        }
    }

    fn read_identifier(&mut self, c: char) -> String {
        let mut ident = String::new();
        ident.push(c);
        while let Some(&c) = self.peek_char() {
            if c.is_alphabetic() {
                ident.push(self.read_char().unwrap());
            } else {
                break;
            }
        }
        ident
    }

    fn read_number(&mut self, c: char) -> String {
        let mut number = String::new();
        number.push(c);
        while let Some(&c) = self.peek_char() {
            if c.is_digit(10) {
                number.push(self.read_char().unwrap());
            } else {
                break;
            }
        }
        number
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        if let Some(c) = self.read_char() {
            match c {
                '=' => Some(Token::Assign),
                '+' => Some(Token::Plus),
                ',' => Some(Token::Comma),
                ';' => Some(Token::Semicolon),
                '(' => Some(Token::LParen),
                ')' => Some(Token::RParen),
                '{' => Some(Token::LBrace),
                '}' => Some(Token::RBrace),
                _ => {
                    if Self::is_letter(c) {
                        Self::lookup_keyword(self.read_identifier(c))
                    } else if c.is_digit(10) {
                        Some(Token::Int(self.read_number(c)))
                    } else {
                        Some(Token::Illegal(c))
                    }
                }
            }
        } else {
            None
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        self.next_token()
    }
}


#[cfg(test)]
mod lexer_tests {
    use ::lexer::Lexer;
    use ::token::Token;
    #[test]
    fn test_next_token() {
        let input = "let five = 5;
                     let ten = 10;

                     let add = fn(x, y) {
                          x + y;
                     };

                     let result = add(five, ten);";

        let mut lexer = Lexer::new(input);
        let expected = [Some(Token::Let),
                        Some(Token::Ident("five".to_owned())),
                        Some(Token::Assign),
                        Some(Token::Int("5".to_owned())),
                        Some(Token::Semicolon),
                        Some(Token::Let),
                        Some(Token::Ident("ten".to_owned())),
                        Some(Token::Assign),
                        Some(Token::Int("10".to_owned())),
                        Some(Token::Semicolon),
                        Some(Token::Let),
                        Some(Token::Ident("add".to_owned())),
                        Some(Token::Assign),
                        Some(Token::Function),
                        Some(Token::LParen),
                        Some(Token::Ident("x".to_owned())),
                        Some(Token::Comma),
                        Some(Token::Ident("y".to_owned())),
                        Some(Token::RParen),
                        Some(Token::LBrace),
                        Some(Token::Ident("x".to_owned())),
                        Some(Token::Plus),
                        Some(Token::Ident("y".to_owned())),
                        Some(Token::Semicolon),
                        Some(Token::RBrace),
                        Some(Token::Semicolon),
                        Some(Token::Let),
                        Some(Token::Ident("result".to_owned())),
                        Some(Token::Assign),
                        Some(Token::Ident("add".to_owned())),
                        Some(Token::LParen),
                        Some(Token::Ident("five".to_owned())),
                        Some(Token::Comma),
                        Some(Token::Ident("ten".to_owned())),
                        Some(Token::RParen),
                        Some(Token::Semicolon),
                        None];

        for e in expected.iter() {
            let t = &lexer.next_token();
            println!("expected {:?}, lexed {:?} ", e, t);
            assert_eq!(e, t);
        }
    }
}
