    // rscheme -- a scheme interpreter written in Rust
    // Copyright (C) {2015) Elizabeth Henry <liz.henry@ouvaton.org>

    // This program is free software; you can redistribute it and/or modify
    // it under the terms of the GNU General Public License as published by
    // the Free Software Foundation; either version 2 of the License, or
    // (at your option) any later version.

use std::result;

pub type Result = result::Result<Token, &'static str>;

#[derive(Debug,Clone)]
pub enum Token {
    Integer (i64),
    Float (f64),
    Ident (String),
    String (String),
    OpeningParen,
    ClosingParen,
    Quote,
    Unquote,
    Quasiquote
}

pub struct Lexer<'a> {
    xs: &'a [char],
    pub tokens: &'a mut Vec<Token>,
    pub n_par: u32
}

impl<'a> Lexer<'a> {
    pub fn new(v:&'a Vec<char>, t:&'a mut Vec<Token>) -> Lexer<'a> {
        Lexer {
            xs: v,
            n_par: 0,
            tokens: t,
        }
    }

    pub fn with_n_par(&mut self, n_par: u32) {
        self.n_par = n_par;
    }

    // Return the number of par (0 if balanced)
    // and fill the tokens vector
    pub fn tokenize(&'a mut self) -> u32 {
        loop {
            if self.xs.len() == 0 {
                return self.n_par;
            }
            else {
                self.read_single_token();
            }
        }
    }
    
    pub fn read_single_token (&mut self) {
        if !self.xs.is_empty() {
            let c = self.xs[0];
            if c.is_whitespace() {
                self.xs = &self.xs[1..];
                self.read_single_token();
            } else {
                match c {
                    '(' => {
                        self.tokens.push(Token::OpeningParen);
                        self.n_par += 1;
                        self.xs = &self.xs[1..];
                    },
                    ')' => {
                        if self.n_par > 0 {
                            self.tokens.push(Token::ClosingParen);
                            self.n_par -= 1;
                            self.xs = &self.xs[1..];
                        } else {
                            error!("Mismatched parenthesis: too many )s");
                            self.xs = &[];
                        }
                    },
                    '\\' => {
                        self.tokens.push(Token::Quote);
                        self.xs = &self.xs[1..];
                    },
                    '`' => {
                        self.tokens.push(Token::Quasiquote);
                        self.xs = &self.xs[1..];
                    },
                    '\'' => {
                        self.tokens.push(Token::Quote);
                        self.xs = &self.xs[1..];
                    },
                    ',' => {
                        self.tokens.push(Token::Unquote);
                        self.xs = &self.xs[1..];
                    },
                    '"' => {
                        self.xs = &self.xs[1..];
                        let mut s = String::new();
                        self.read_string(&mut s);
                    },
                    '0' ... '9' => {
                        let mut s = String::new();
                        self.read_number(&mut s,0);
                    },
                    '.' => {
                        let mut s = String::new();
                        s.push('.');
                        self.xs = &self.xs[1..];
                        self.read_number(&mut s,1);
                    },
                    ';' => {
                        self.xs = &self.xs[1..];
                        self.read_comment();
                    },
                    _ => {
                        let mut s = String::new();
                        self.read_ident(&mut s);
                    }
                }
            }
        }
    }
    
    fn finish_number(&mut self, s:&String, n_dot:u8) {
        if n_dot == 0 { // integer
            match s.parse::<i64>() {
                Ok(x) => self.tokens.push(Token::Integer(x)),
                Err(_) => {
                    error!("Error parsing 'integer': {}", s.clone());
                    self.xs = &[];
                }
            }
        } else { // float
            match s.parse::<f64>() {
                Ok(x) => self.tokens.push(Token::Float(x)),
                Err(_) => {
                    error!("Error parsing 'float': {}", s.clone());
                    self.xs = &[];
                }
            }
        }
    }
    
    fn read_number(&mut self,s:&mut String, n_dot:u8) {
        if self.xs.len() == 0 {
            self.finish_number(s, n_dot);
            return;
        } 
        
        let c = self.xs[0];
        if c.is_whitespace() {
            self.xs = &self.xs[1..];
            self.finish_number(s, n_dot);
            return;
        }

        match c {
            '0' ... '9' => {
                s.push(c);
                self.xs = &self.xs[1..];
                self.read_number(s, n_dot);
            }
            '.' => {
                if n_dot == 0 {
                    s.push('.');
                    self.xs = &self.xs[1..];
                    self.read_number(s,1);
                } else {
                    error!("Lexer: Invalid number: contains more than one dot");
                    self.xs = &[];
                }
            },
            '('|')'|';' => self.finish_number(s,n_dot),
            _ => {
                error!("Lexer: Invalid character in a number: {}", c);
                self.xs = &[];
            }
        }
    }
        
    fn read_string(&mut self,s:&mut String) {
        if self.xs.len() == 0 {
                error!("Lexer error: can't find closing quote");
        } else {
            let c = self.xs[0];
            match c {
                '"' => {
                    self.tokens.push(Token::String(s.clone()));
                    self.xs = &self.xs[1..];
                },
                '\\' => {
                    if self.xs.len() == 1 {
                        error!("Lexer error: can't finish lexing string");
                    } else {
                        let c2 = self.xs[1];
                        self.xs = &self.xs[2..];
                        match c2 {
                            '\\' => {
                                s.push('\\');
                                self.read_string(s);
                            },
                            '"' => {
                                s.push('"');
                                self.read_string(s);
                            },
                            'n' => {
                                s.push('\n');
                                self.read_string(s);
                            },
                            _ => {
                                error!("Unrecognized escape character \\{}", c2);
                                self.xs = &[];
                            }
                        }
                    }
                },
                _ => {
                    s.push(c);
                    self.xs = &self.xs[1..];
                    self.read_string(s);
                }
            }
        }
    }

    fn read_comment(&mut self) {
        if self.xs.len() == 0 {
            return;
        }
        let c = self.xs[0];
        if c == '\n' {
            return;
        } else {
            self.xs = &self.xs[1..];
            self.read_comment();
        }
    }
        
        
    
    fn read_ident(&mut self, s:&mut String) {
        if self.xs.len() == 0 {
            self.tokens.push(Token::Ident(s.clone()));
            return;
        } 
        let c = self.xs[0];
        if c.is_whitespace() {
            self.xs = &self.xs[1..];
            self.tokens.push(Token::Ident(s.clone()));
            return;
        }

        match c {
            '(' | ')'|';' => self.tokens.push(Token::Ident(s.clone())),
            _ => {
                s.push(c);
                self.xs = &self.xs[1..];
                self.read_ident(s)
            }
        }
    }
}



