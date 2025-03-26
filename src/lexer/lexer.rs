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

fn push(type_name: String, type_value: String, array: &mut Vec<Token>) {
    array.push(Token {
        token_type: type_name,
        value: type_value,
    });
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
                            push("keyword".to_string(), token.iter().collect(), &mut tokens);
                        } else if is_bool_identifier(token.iter().collect()) {
                            push("bool".to_string(), token.iter().collect(), &mut tokens);
                        } else {
                            push(
                                "identifier".to_string(),
                                token.iter().collect(),
                                &mut tokens,
                            );
                        }
                    }
                    WordType::Operator => {
                        push("operator".to_string(), token.iter().collect(), &mut tokens);
                    }
                    WordType::Separator => {
                        push("separator".to_string(), token.iter().collect(), &mut tokens);
                    }
                    WordType::Literal(ref t) => match *t {
                        LiteralType::Integer(ref t) => match *t {
                            BinaryType::Hex => {
                                push("hex".to_string(), token.iter().collect(), &mut tokens);
                            }
                            BinaryType::Oct => {
                                push("oct".to_string(), token.iter().collect(), &mut tokens);
                            }
                            BinaryType::Dec => {
                                push("dec".to_string(), token.iter().collect(), &mut tokens);
                            }
                            BinaryType::Unknown => {
                                push("unknown".to_string(), token.iter().collect(), &mut tokens);
                            }
                        },
                        LiteralType::Float => {
                            push("float".to_string(), token.iter().collect(), &mut tokens);
                        }
                        LiteralType::Char => {
                            push("char".to_string(), token.iter().collect(), &mut tokens);
                        }
                        LiteralType::Unknown => {
                            push("unknown".to_string(), token.iter().collect(), &mut tokens);
                        }
                    },
                    WordType::String => {
                        push("string".to_string(), token.iter().collect(), &mut tokens);
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
                        push("Unknown".to_string(), token.iter().collect(), &mut tokens);
                    }
                }
                token.clear();
                state = state_init();
            }
            State::Unaccepted => {
                let pos = f.position();
                token.push(c);
                while let Some(c) = f.get_char() {
                    f.update_pointer(1);
                    f.update_position(c);
                    if is_separator(c) || is_invisible_char(c as u8) {
                        break;
                    }
                    token.push(c);
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
