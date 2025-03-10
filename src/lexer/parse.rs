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

    while let Some(c) = f.get_char() {
        println!("{:?}->{:#?}", c, state);
        let nc = f.look_forward();
        if let Some(next) = translation(c, nc, state) {
            state = next;
        } else {
            panic!("Error: unexpected state!");
        }
        match state {
            State::Accepted(ref t) => {
                println!("{:?}", token);
                match *t {
                    WordType::Identifier => {
                        if is_reserved_word(token.iter().collect()) {
                            tokens.push(Token {
                                token_type: "keyword".to_string(),
                                value: token.iter().collect(),
                            });
                        } else {
                            tokens.push(Token {
                                token_type: "identifier".to_string(),
                                value: token.iter().collect(),
                            });
                        }
                    }
                    WordType::Keyword => {
                        tokens.push(Token {
                            token_type: "keyword".to_string(),
                            value: token.iter().collect(),
                        });
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
                            BinaryType::Bin => {
                                tokens.push(Token {
                                    token_type: "bin".to_string(),
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
                        LiteralType::Bool => {
                            tokens.push(Token {
                                token_type: "bool".to_string(),
                                value: token.iter().collect(),
                            });
                        }
                    },
                    WordType::String => {
                        tokens.push(Token {
                            token_type: "string".to_string(),
                            value: token.iter().collect(),
                        });
                    }
                    WordType::Comment => {
                        tokens.push(Token {
                            token_type: "comment".to_string(),
                            value: token.iter().collect(),
                        });
                    }
                    WordType::Unknown => {
                        tokens.push(Token {
                            token_type: "unknown".to_string(),
                            value: token.iter().collect(),
                        });
                    }
                }
            }
            State::Unaccepted => {
                let info = f.position();
                panic!("Error->position ({}, {})!", info.0, info.1);
            }
            State::Handling(_) => {
                token.push(c);
                if (c as u8) == 0xa {
                    f.add_row();
                    f.init_col();
                } else {
                    f.add_col();
                }
                f.update_pointer(1);
            }
            State::Init => {
                // donothing
            }
        }
    }
    tokens
}

pub fn print_tokens(tokens: Vec<Token>) {
    for token in tokens {
        println!("{:?} {:?}", token.token_type, token.value);
    }
}
