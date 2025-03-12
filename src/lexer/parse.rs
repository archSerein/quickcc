use crate::lexer::NEWLINE;

use super::file::Source;
use super::helper::*;
use std::panic;
use std::vec::Vec;

#[derive(Debug)]
pub struct Token {
    token_type: String,
    value: String,
}

pub fn run(filepath: &str) -> Vec<Token> {
    let mut f = Source::new(filepath);
    let mut tokens = Vec::new();
    let mut state = State::Init;
    let mut token: Vec<char> = Vec::new();
    let mut err_info: Vec<String> = Vec::new();

    while let Some(c) = f.get_char() {
        let nc = f.look_forward();
        if let Some(next) = transition(c, nc, state) {
            state = next;
        } else {
            panic!("Error: unexpected state!");
        }
        f.update_pointer(1);
        // println!("{:?} {:?}", state, c);
        match state {
            State::Accepted(ref t) => {
                let need_push = c as u8 != super::NEWLINE && c as u8 != super::SPACE;
                if need_push {
                    token.push(c);
                }
                f.update_position(c);
                match *t {
                    WordType::Identifier => {
                        if is_reserved_word(token.iter().collect()) {
                            tokens.push(Token {
                                token_type: "keyword".to_string(),
                                value: token.iter().collect(),
                            });
                        } else if is_bool_identifier(token.iter().collect()) {
                            tokens.push(Token {
                                token_type: "bool".to_string(),
                                value: token.iter().collect(),
                            });
                        } else {
                            tokens.push(Token {
                                token_type: "identifier".to_string(),
                                value: token.iter().collect(),
                            });
                        }
                    }
                    WordType::Operator => {
                        tokens.push(Token {
                            token_type: "operator".to_string(),
                            value: token.iter().collect(),
                        });
                    }
                    WordType::Separator => {
                        tokens.push(Token {
                            token_type: "separator".to_string(),
                            value: token.iter().collect(),
                        });
                    }
                    WordType::Literal(ref t) => match *t {
                        LiteralType::Integer(ref t) => match *t {
                            BinaryType::Hex => {
                                tokens.push(Token {
                                    token_type: "hex".to_string(),
                                    value: token.iter().collect(),
                                });
                            }
                            BinaryType::Oct => {
                                tokens.push(Token {
                                    token_type: "oct".to_string(),
                                    value: token.iter().collect(),
                                });
                            }
                            BinaryType::Dec => {
                                tokens.push(Token {
                                    token_type: "dec".to_string(),
                                    value: token.iter().collect(),
                                });
                            }
                            BinaryType::Unknown => {
                                tokens.push(Token {
                                    token_type: "unknown".to_string(),
                                    value: token.iter().collect(),
                                });
                            }
                        },
                        LiteralType::Float => {
                            tokens.push(Token {
                                token_type: "float".to_string(),
                                value: token.iter().collect(),
                            });
                        }
                        LiteralType::Char => {
                            tokens.push(Token {
                                token_type: "char".to_string(),
                                value: token.iter().collect(),
                            });
                        }
                        LiteralType::Unknown => {
                            tokens.push(Token {
                                token_type: "unknown".to_string(),
                                value: token.iter().collect(),
                            });
                        }
                    },
                    WordType::String => {
                        while let Some(c) = f.get_char() {
                            f.update_pointer(1);
                            token.push(c);
                            if c == '"' {
                                break;
                            }
                        }
                        tokens.push(Token {
                            token_type: "string".to_string(),
                            value: token.iter().collect(),
                        });
                    }
                    WordType::Comment => {
                        let comment_type = token[1] == '/';
                        if comment_type {
                            while let Some(c) = f.get_char() {
                                f.update_pointer(1);
                                if c as u8 == NEWLINE {
                                    f.update_position(c);
                                    break;
                                }
                            }
                        } else {
                            let mut comment_end = false;
                            while let Some(c) = f.get_char() {
                                f.update_pointer(1);
                                if c == '*' {
                                    comment_end = true;
                                } else if comment_end && c == '/' {
                                    break;
                                } else {
                                    f.update_position(c);
                                    comment_end = false;
                                }
                            }
                        }
                    }
                    WordType::Unknown => {
                        tokens.push(Token {
                            token_type: "unknown".to_string(),
                            value: token.iter().collect(),
                        });
                    }
                }
                token.clear();
                state = state_init();
            }
            State::Unaccepted => {
                let pos = f.position();
                token.push(c);
                while let Some(c) = f.get_char() {
                    token.push(c);
                    f.update_pointer(1);
                    f.update_position(c);
                    if is_separator(c) {
                        break;
                    }
                }
                let info = format!(
                    "Error->position ({}, {})! {:?}",
                    pos.0,
                    pos.1,
                    token.iter().collect::<String>()
                );
                err_info.push(info);
                token.clear();
                state = state_init();
            }
            State::Handling(_) => {
                f.update_position(c);
                token.push(c);
                if is_hex_format(c, nc, state) {
                    let next_c = nc.unwrap_or(' ');
                    token.push(next_c);
                    f.update_pointer(1);
                }
            }
            State::Init => {
                f.update_position(c);
            }
        }
    }
    print_err_info(err_info);
    tokens
}

pub fn is_hex_format(c: char, nc: Option<char>, state: State) -> bool {
    match state {
        State::Handling(WordType::Literal(LiteralType::Integer(BinaryType::Hex))) => {
            let next_c = nc.unwrap_or(' ');
            c == '0' && next_c == 'x'
        }
        _ => false,
    }
}

pub fn print_tokens(tokens: Vec<Token>) {
    for token in tokens {
        println!("{:?} {:?}", token.token_type, token.value);
    }
}

pub fn print_err_info(err_info: Vec<String>) {
    for info in err_info {
        println!("{:?}", info);
    }
}
