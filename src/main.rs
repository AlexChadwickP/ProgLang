// Warning silencing
#![allow(dead_code, non_snake_case)]

use std::collections::VecDeque;
use std::fmt;
use std::io::{stdin, stdout, Write};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, PartialEq, EnumIter)]
enum TokenType {
    // Types
    Int,
    Float,
    String,
    Keyword,
    // Arithmetic Operators
    Plus,
    Multiply,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    token_value: String,
}

impl Token {
    fn new(t_type: TokenType, t_value: String) -> Token {
        Token {
            token_type: t_type,
            token_value: t_value,
        }
    }

    fn print(&self) {
        println!("{:?}:{}", self.token_type, self.token_value);
    }
}

struct Error {
    name: String,
    description: String,
}

impl Error {
    fn new(name: &str, description: &str) -> Error {
        Error {
            name: String::from(name),
            description: String::from(description),
        }
    }

    fn throw(&self) {
        println!("{}: {}", self.name, self.description);
        std::process::exit(1);
    }
}

struct Lexer {
    src: String,
    current_position: usize,
    current_character: char,
}

impl Lexer {
    fn new(source: String) -> Lexer {
        let first_character = source.chars().nth(0).unwrap();
        Lexer {
            src: source,
            current_position: 0,
            current_character: first_character,
        }
    }

    fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        while self.current_character != '\0' {
            if self.current_character.is_numeric() {
                tokens.push(self.match_number());
            }
            if self.current_character == '+' {
                tokens.push(Token {
                    token_type: TokenType::Plus,
                    token_value: String::new(),
                });
            }
            if self.current_character == '*' {
                tokens.push(Token {
                    token_type: TokenType::Multiply,
                    token_value: String::new(),
                });
            }
            if self.current_character == '"' {
                tokens.push(self.match_string());
            }
            if (self.current_character != '+'
                && self.current_character != '"'
                && !self.current_character.is_numeric())
                && !self.current_character.is_whitespace()
            {
                tokens.push(self.match_keyword());
            }
            self.advance();
        }

        return tokens;
    }

    fn peek(&mut self, offset: usize) -> char {
        return match self
            .src
            .chars()
            .nth((self.current_position + offset) as usize)
        {
            Some(character) => character,
            None => '\0',
        };
    }

    fn advance(&mut self) {
        self.current_position += 1;
        self.current_character = match self.src.chars().nth(self.current_position as usize) {
            Some(character) => character,
            None => '\0',
        };
    }

    fn match_number(&mut self) -> Token {
        let mut has_dot: bool = false;
        let mut number: String = String::new();

        number += &*self.current_character.to_string();

        while self.peek(1).is_numeric() || self.peek(1) == '.' {
            self.advance();
            if has_dot && self.current_character == '.' {
                Error::new("IllegalCharError", "Found an extra dot").throw()
            } else if self.current_character == '.' {
                has_dot = true;
            }
            number += &*self.current_character.to_string();
        }

        self.advance();

        return if has_dot {
            Token::new(TokenType::Float, number)
        } else {
            Token::new(TokenType::Int, number)
        };
    }

    fn match_string(&mut self) -> Token {
        let mut string = String::new();

        while self.peek(1).is_ascii() && self.peek(1) != '"' {
            self.advance();
            string += &*self.current_character.to_string();
        }

        self.advance();

        return Token::new(TokenType::String, string);
    }

    fn match_keyword(&mut self) -> Token {
        let mut keyword: String = String::from(self.current_character);

        while !self.peek(1).is_whitespace() {
            self.advance();
            keyword += &*self.current_character.to_string();
        }

        return Token::new(TokenType::Keyword, keyword);
    }
}

struct Runner {
    token_stack: VecDeque<Token>,
}

impl Runner {
    fn new(stack: VecDeque<Token>) -> Runner {
        Runner { token_stack: stack }
    }

    fn start(&mut self) {
        while !self.token_stack.is_empty() {
            // DEBUG: println!("{:?}", self.token_stack.last().unwrap());
            let token: Token = self.token_stack.pop_back().unwrap();
            // Addition!
            if token.token_type.to_string() == String::from("Plus") {
                let token_to_add = self.add();
                self.token_stack.push_front(token_to_add);
            } else if token.token_type.to_string() == String::from("Keyword") {
                self.handle_keyword(token);
            }
        }
    }

    fn handle_keyword(&mut self, token: Token) {
        let keyword = token.token_value;
        match &keyword[..] {
            "puts" => self.puts(),
            _ => Error::new(
                "Unknown keyword error",
                &format!("No such keyword: {}", keyword)[..],
            )
            .throw(),
        }
    }

    fn puts(&mut self) {
        let valueToPrint: String = self.token_stack.pop_back().unwrap().token_value;

        println!("{}", valueToPrint);
    }

    fn add(&mut self) -> Token {
        let mut first = self.token_stack.pop_front().unwrap();
        let mut second = self.token_stack.pop_front().unwrap();

        if second.token_type == TokenType::Plus {
            second = self.add();
        }

        if first.token_type != second.token_type {
            Error::new(
                "Mismatched types",
                "Cannot add on 2 values of different types",
            )
            .throw();
        }

        let first_num = first.token_value;
        let second_num = second.token_value;

        let result: usize =
            first_num.parse::<usize>().unwrap() + second_num.parse::<usize>().unwrap();

        println!("{}", result);

        Token::new(first.token_type, result.to_string())
    }
}

fn get_input(msg: &str) -> String {
    print!("{}", msg);
    let mut s = String::new();
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("How on earth did you mess up a string");

    return s;
}

fn main() {
    loop {
        let input = get_input("> ");
        let mut lexer: Lexer = Lexer::new(String::from(input));
        let tokens: VecDeque<Token> = VecDeque::from(lexer.tokenize());

        let mut runner: Runner = Runner::new(tokens);
        runner.start();
    }
}
