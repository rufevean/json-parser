use std::any::type_name;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use super::minify;
use super::parser;
#[derive(Debug, PartialEq)]
pub enum Token {
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Assign,
    Comma,
    String(String),
    Number(usize),
    NewLine,
    Boolean(bool),
    EOF,
}

impl Lexer {
    fn consume(&mut self) {
        self.position += 1;
    }
    fn next_token(&mut self) -> Token {
        if let Some(c) = self.current_char() {
            match c {
                b'{' => self.braces(),
                b'}' => self.braces(),
                b'[' => self.brackets(),
                b']' => self.brackets(),
                b':' => self.assign(),
                b'\n' => self.new_line(),
                b't' | b'f' => self.is_boolean(),
                b'"' => self.is_string(),
                b',' => self.comma(),
                b'0'..=b'9' => self.number(),
                _ => self.end_of_file(),
            }
        } else {
            return self.end_of_file();
        }
    }
    fn end_of_file(&mut self) -> Token {
        return Token::EOF;
    }

    fn number(&mut self) -> Token {
        while let Some(c) = self.current_char() {
            if c.is_ascii_digit() {
                self.consume();
            } else {
                break;
            }
        }

        Token::Number(
            self.input[self.position - 1..self.position]
                .parse::<u32>()
                .unwrap() as usize,
        )
    }

    fn comma(&mut self) -> Token {
        if self.current_char().unwrap() == b',' {
            self.consume();
            return Token::Comma;
        } else {
            return self.end_of_file();
        }
    }
    fn new_line(&mut self) -> Token {
        while self.current_char().unwrap() != b'\n' {
            self.consume();
            return Token::NewLine;
        }
        return Token::EOF;
    }

    fn braces(&mut self) -> Token {
        if self.current_char().unwrap() == b'{' {
            self.consume();
            return Token::LBrace;
        } else if self.current_char().unwrap() == b'}' {
            self.consume();
            return Token::RBrace;
        } else {
            return self.end_of_file();
        }
    }

    fn brackets(&mut self) -> Token {
        if self.current_char().unwrap() == b'[' {
            self.consume();
            return Token::LBracket;
        } else if self.current_char().unwrap() == b']' {
            self.consume();
            return Token::RBracket;
        } else {
            return self.end_of_file();
        }
    }
    fn assign(&mut self) -> Token {
        if self.current_char().unwrap() == b':' {
            self.consume();
            return Token::Assign;
        } else {
            return self.end_of_file();
        }
    }
    fn is_string(&mut self) -> Token {
        if self.current_char().unwrap() == b'"' {
            let start = self.position;
            self.consume();
            while self.current_char().unwrap() != b'"' {
                self.consume();
            }
            self.consume();
            let value = self.input[start..self.position].to_string();
            return Token::String(value);
        } else {
            return self.end_of_file();
        }
    }

    fn current_char(&self) -> Option<u8> {
        if self.position >= self.input.len() {
            None
        } else {
            Some(self.input.as_bytes()[self.position])
        }
    }
    fn is_boolean(&mut self) -> Token {
        let booleans = ["true", "false"];
        let start = self.position;
        while self.current_char().unwrap().is_ascii_alphabetic() {
            self.consume();
        }
        let input_boolean = &self.input[start..self.position];
        if booleans.contains(&input_boolean) {
            return Token::Boolean(input_boolean == "true");
        } else {
            return self.end_of_file();
        }
    }
}
struct Lexer {
    input: String,
    position: usize,
}

pub fn read_contents(path: &Path) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

pub fn create_lexer(input: String) {
    let mut lexer = Lexer {
        input: input.to_string(),
        position: 0,
    };
    let mut token_vec: Vec<Token> = Vec::new();
    let mut _value_vec: Vec<String> = Vec::new();
    loop {
        let token = lexer.next_token();

        if token == Token::EOF {
            break;
        }
        token_vec.push(token);
    }
    parser::parse(token_vec);
}

fn type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}

// main
pub fn file_to_lexer(input: &Path) {
    let contents = read_contents(input);
    let minified_contents = minify::minify(&contents);
    create_lexer(minified_contents);
}
