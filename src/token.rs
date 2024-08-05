use std::borrow::Borrow;
use std::collections::HashMap;
use std::iter::Peekable;
use std::ops::DerefMut;
use std::str::Chars;
use std::string;
use std::fmt;

use regex::Regex;

const ANONYMOUS_FUNCTION_NAME: &str = "anonymous";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    Identifier,
    int_var,
    str_var,
    String,
    Number,
    Plus,
    Minus,
    Slash,
    Star,
    Semicolon,
    LParen,
    RParen,
    Equals,
    Comment,
    Var,
    Display,
    EOF,
    DisplayStr,
    DisplayInt,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Str,
    Unknown,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Int => write!(f, "Int"),
            Type::Str => write!(f, "Str"),
            Type::Unknown => write!(f, "Unknown"),
        }
    }
}


#[derive(Debug,Clone)]
enum Literal {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,
}

#[derive(Debug,Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    literal: Option<Literal>,
    pub line: u32,
    pub var_type: Option<Type>,
}

impl Token {
    pub fn newtok(
        token_type: TokenType,
        lexeme: String,
        literal: Option<Literal>,
        line: u32,
        var_type: Option<Type>,
    ) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
            var_type,
        }
    }
}

pub struct Lexer<'a> {
    source: Option<Peekable<Chars<'a>>>,
    tokens: Vec<Token>,
    line: u32,
    keywords: HashMap<String, TokenType>,
    types: HashMap<String, Type>,
}

impl<'a> Lexer<'a> {
    pub fn new() -> Self {
        let mut lexer = Lexer {
            source: None,
            tokens: Vec::new(),
            line: 1,
            keywords: HashMap::new(),
            types: HashMap::new(),
        };
        lexer.keywords.insert("VARint".to_string(), TokenType::int_var);
        lexer.keywords.insert("VARstr".to_string(), TokenType::str_var);
        lexer.keywords.insert("DISPLAY".to_string(), TokenType::Display);
        lexer.keywords.insert("DISPLAYnumeric".to_string(), TokenType::DisplayInt);
        lexer.keywords.insert("DISPLAYstring".to_string(), TokenType::DisplayStr);
        lexer.keywords.insert("END".to_string(), TokenType::EOF);
        
        
        
        lexer
    }

    pub fn tokenize(&mut self, input: &'a str) {
        self.source = Some(input.chars().peekable());
        let mut buffer = String::new();

        while let Some(ch) = self.source.as_mut().unwrap().next() {
            match ch {
                '+' | '-' | '*' | '/' | '=' | '(' | ')' | ';' => {
                    self.handle_buffer(&mut buffer);
                    self.tokens.push(Token::newtok(
                        match ch {
                            '+' => TokenType::Plus,
                            '-' => TokenType::Minus,
                            '*' => TokenType::Star,
                            '/' => TokenType::Slash,
                            '=' => TokenType::Equals,
                            '(' => TokenType::LParen,
                            ')' => TokenType::RParen,
                            ';' => TokenType::Semicolon,
                            _ => unreachable!(),
                        },
                        ch.to_string(),
                        None,
                        self.line,
                        None,
                    ));
                    if ch == ';' {
                        self.line += 1;
                    }
                }
                '"' => self.string_handle(),
                ch if ch.is_whitespace() => {
                    self.handle_buffer(&mut buffer);
                }
                ch if ch.is_alphanumeric() || ch == '_' => buffer.push(ch),
                _ => {
                    self.handle_buffer(&mut buffer);
                }
            }
        }
        self.handle_buffer(&mut buffer);
    }

    fn handle_buffer(&mut self, buffer: &mut String) {
        if !buffer.is_empty() {
            let re_integer = Regex::new(r"^\d+$").unwrap();
            let re_identifier = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap();

            match buffer.as_str() {
                buffer if self.keywords.contains_key(buffer) => {
                    let token_type = self.keywords.get(buffer).unwrap().clone();
                    let literal = Some(Literal::String(buffer.to_string()));
                    let var_type = match token_type {
                        TokenType::int_var => Some(Type::Int),
                        TokenType::str_var => Some(Type::Str),
                        _ => None,
                    };
                    self.tokens.push(Token::newtok(
                        token_type,
                        buffer.to_string(),
                        literal,
                        self.line,
                        var_type,
                    ));
                }
                buffer if self.types.contains_key(buffer) => {
                    let var_type = self.types.get(buffer).unwrap().clone();
                    self.tokens.push(Token::newtok(
                        TokenType::Identifier,
                        buffer.to_string(),
                        Some(Literal::String(buffer.to_string())),
                        self.line,
                        Some(var_type),
                    ));
                }
                _ if re_integer.is_match(buffer) => {
                    let int_value = buffer.parse::<i64>().unwrap();
                    self.tokens.push(Token::newtok(
                        TokenType::Number,
                        buffer.clone(),
                        Some(Literal::Integer(int_value)),
                        self.line,
                        None,
                    ));
                }
                _ if re_identifier.is_match(buffer) => {
                    self.tokens.push(Token::newtok(
                        TokenType::Identifier,
                        buffer.clone(),
                        Some(Literal::String(buffer.clone())),
                        self.line,
                        None,
                    ));
                }
                _ => {
                    println!("Unexpected character");
                }
            }
            buffer.clear();
        }
    }

    fn string_handle(&mut self) {
        let mut string_content = String::new();
        while let Some(cha) = self.source.as_mut().unwrap().next() {
            match cha {
                '"' => {
                    self.tokens.push(Token::newtok(
                        TokenType::String,
                        string_content.clone(),
                        Some(Literal::String(string_content.clone())),
                        self.line,
                        Some(Type::Str),
                    ));
                    return;
                }
                _ => string_content.push(cha),
            }
        }
        println!("Error: Unterminated string literal");
    }

    pub fn print_tok(&mut self) {
        println!("{:#?}", self.tokens);
    }

    pub fn return_tok(&mut self) -> Vec<Token> {
        self.tokens.clone()
    }
}
