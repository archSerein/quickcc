use crate::lexer::NEWLINE;
use crate::utils::helper::print_err_info;
use crate::utils::types::PhraseType;

use super::file::Source;
use super::helper::*;
use std::panic;
use std::vec::Vec;

#[derive(Debug, Clone)]
pub struct Token {
    pub pos: usize,
    pub types: PhraseType,
    pub value: String,
}

fn push(pos_cow: usize, types: PhraseType, token_value: String, array: &mut Vec<Token>) {
    array.push(Token {
        pos: pos_cow,
        types,
        value: token_value,
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
                let pos_cow = f.position().0 as usize;
                match *t {
                    WordType::Identifier => {
                        if is_reserved_word(token.iter().collect()) {
                            push(
                                pos_cow,
                                PhraseType::Keyword,
                                token.iter().collect(),
                                &mut tokens,
                            );
                        } else if is_bool_identifier(token.iter().collect()) {
                            push(
                                pos_cow,
                                PhraseType::Bool,
                                token.iter().collect(),
                                &mut tokens,
                            );
                        } else {
                            push(
                                pos_cow,
                                PhraseType::Identifier,
                                token.iter().collect(),
                                &mut tokens,
                            );
                        }
                    }
                    WordType::Operator => {
                        push(
                            pos_cow,
                            PhraseType::Operator,
                            token.iter().collect(),
                            &mut tokens,
                        );
                    }
                    WordType::Separator => {
                        push(
                            pos_cow,
                            PhraseType::Separator,
                            token.iter().collect(),
                            &mut tokens,
                        );
                    }
                    WordType::Literal(ref t) => match *t {
                        LiteralType::Integer(ref t) => match *t {
                            BinaryType::Hex => {
                                push(
                                    pos_cow,
                                    PhraseType::Hex,
                                    token.iter().collect(),
                                    &mut tokens,
                                );
                            }
                            BinaryType::Oct => {
                                push(
                                    pos_cow,
                                    PhraseType::Oct,
                                    token.iter().collect(),
                                    &mut tokens,
                                );
                            }
                            BinaryType::Dec => {
                                push(
                                    pos_cow,
                                    PhraseType::Dec,
                                    token.iter().collect(),
                                    &mut tokens,
                                );
                            }
                            BinaryType::Unknown => {
                                push(
                                    pos_cow,
                                    PhraseType::Unknown,
                                    token.iter().collect(),
                                    &mut tokens,
                                );
                            }
                        },
                        LiteralType::Float => {
                            push(
                                pos_cow,
                                PhraseType::Float,
                                token.iter().collect(),
                                &mut tokens,
                            );
                        }
                        LiteralType::Char => {
                            push(
                                pos_cow,
                                PhraseType::Char,
                                token.iter().collect(),
                                &mut tokens,
                            );
                        }
                        LiteralType::Unknown => {
                            push(
                                pos_cow,
                                PhraseType::Unknown,
                                token.iter().collect(),
                                &mut tokens,
                            );
                        }
                    },
                    WordType::String => {
                        push(
                            pos_cow,
                            PhraseType::String,
                            token.iter().collect(),
                            &mut tokens,
                        );
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
                        push(
                            pos_cow,
                            PhraseType::Unknown,
                            token.iter().collect(),
                            &mut tokens,
                        );
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
