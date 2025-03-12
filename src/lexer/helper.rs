use super::SPACE;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryType {
    Hex,
    Oct,
    Dec,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LiteralType {
    Integer(BinaryType),
    Float,
    Char,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WordType {
    Identifier,
    Operator,
    Separator,
    Literal(LiteralType),
    String,
    Comment,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum State {
    Accepted(WordType),
    Unaccepted,
    Handling(WordType),
    Init,
}

pub fn state_init() -> State {
    State::Init
}
pub fn transition(c: char, nc: Option<char>, state: State) -> Option<State> {
    match state {
        State::Init => {
            if is_invisible_char(c as u8) {
                Some(State::Init)
            } else if is_separator(c) {
                Some(State::Accepted(WordType::Separator))
            } else if is_operator(c) {
                let next_c = nc.unwrap_or(' ');
                if !is_operator(next_c) {
                    Some(State::Accepted(WordType::Operator))
                } else if let Some(next_type) =
                    WordType::cal_word_type(c, next_c, WordType::Operator)
                {
                    Some(State::Handling(next_type))
                } else {
                    Some(State::Unaccepted)
                }
            } else if is_valid_char(c) {
                let next_c = nc.unwrap_or(' ');
                let next_type = WordType::cal_word_type(c, next_c, WordType::Unknown);
                if next_c as u8 == SPACE || is_separator(next_c) || is_operator(next_c) {
                    if let Some(next) = next_type {
                        Some(State::Accepted(next))
                    } else {
                        Some(State::Unaccepted)
                    }
                } else if let Some(next) = next_type {
                    Some(State::Handling(next))
                } else {
                    Some(State::Unaccepted)
                }
            } else if c as u8 == 0x22 {
                Some(State::Handling(WordType::String))
            } else if c as u8 == 0x27 {
                Some(State::Handling(WordType::Literal(LiteralType::Char)))
            } else {
                Some(State::Unaccepted)
            }
        }
        State::Handling(current_type) => {
            let next_c = nc.unwrap_or(' '); // 如果 None，则默认 ' '
            if is_separator(next_c) || next_c as u8 == SPACE {
                let next_type = WordType::cal_word_type(c, next_c, current_type);
                if let Some(next) = next_type {
                    Some(State::Accepted(next))
                } else {
                    Some(State::Unaccepted)
                }
            } else {
                let next_type = WordType::cal_word_type(c, next_c, current_type);
                match next_type {
                    Some(next) => match next {
                        WordType::Unknown => Some(State::Accepted(current_type)),
                        _ => Some(State::Handling(next)),
                    },
                    None => Some(State::Unaccepted),
                }
            }
        }

        State::Accepted(_) | State::Unaccepted => Some(State::Init),
    }
}

pub fn is_invisible_char(c: u8) -> bool {
    match c {
        // 空格
        32 => true,
        // \r\n
        10 | 13 => true,
        // tab
        9 | 11 => true,
        // other
        _ => false,
    }
}

pub fn is_separator(c: char) -> bool {
    matches!(c, '(' | ')' | '{' | '}' | '[' | ']' | ';' | ',' | ':')
}

pub fn is_operator(c: char) -> bool {
    matches!(
        c,
        '+' | '-' | '*' | '/' | '%' | '=' | '!' | '>' | '<' | '&' | '|' | '^' | '~'
    )
}

pub fn is_valid_char(c: char) -> bool {
    let is_alpha = c.is_ascii_alphabetic();
    let is_digit = c.is_ascii_digit();
    let is_special = matches!(c, '_' | '$');
    is_alpha || is_digit || is_special
}

impl LiteralType {
    pub fn cal_literal_type(c: char, nc: char, state: LiteralType) -> Option<LiteralType> {
        match state {
            LiteralType::Integer(ref t) => {
                if let Some(next_state) = BinaryType::cal_binary_type(c, nc, *t) {
                    Some(LiteralType::Integer(next_state))
                } else if c == '.' {
                    Some(LiteralType::Float)
                } else {
                    None
                }
            }
            LiteralType::Float => match c {
                '0'..='9' => Some(LiteralType::Float),
                _ => None,
            },
            LiteralType::Char => {
                if c.is_ascii() && c != '\'' {
                    Some(LiteralType::Char)
                } else if c == '\'' {
                    Some(LiteralType::Unknown)
                } else {
                    None
                }
            }
            LiteralType::Unknown => match c {
                '0'..='9' => BinaryType::cal_binary_type(c, nc, BinaryType::Unknown)
                    .map(LiteralType::Integer),
                '\'' => Some(LiteralType::Char),
                _ => None,
            },
        }
    }
}

impl WordType {
    pub fn cal_word_type(c: char, nc: char, state: WordType) -> Option<WordType> {
        match state {
            WordType::Unknown => match c {
                'a'..='z' | 'A'..='Z' | '_' => Some(WordType::Identifier),
                '0'..='9' => LiteralType::cal_literal_type(c, nc, LiteralType::Unknown)
                    .map(WordType::Literal),
                _ => None,
            },
            WordType::Identifier => match c {
                'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => Some(WordType::Identifier),
                _ => None,
            },
            WordType::Literal(ref t) => {
                LiteralType::cal_literal_type(c, nc, *t).map(WordType::Literal)
            }
            WordType::String => Some(WordType::String),
            WordType::Comment => Some(WordType::Comment),
            WordType::Operator => {
                if is_comment(c, nc) {
                    Some(WordType::Comment)
                } else if is_operator(c) {
                    if !is_operator(nc) {
                        Some(WordType::Unknown)
                    } else {
                        Some(WordType::Operator)
                    }
                } else {
                    None
                }
            }
            WordType::Separator => Some(WordType::Separator),
        }
    }
}

impl BinaryType {
    pub fn cal_binary_type(c: char, nc: char, state: BinaryType) -> Option<BinaryType> {
        match state {
            BinaryType::Hex => match c {
                '0'..='9' | 'a'..='f' | 'A'..='F' => Some(BinaryType::Hex),
                _ => None,
            },
            BinaryType::Oct => match c {
                '0'..='7' => Some(BinaryType::Oct),
                _ => None,
            },
            BinaryType::Dec => {
                if c.is_numeric() {
                    Some(BinaryType::Dec)
                } else {
                    None
                }
            }
            BinaryType::Unknown => match c {
                '1'..='9' => {
                    if nc.is_numeric() || is_separator(nc) {
                        Some(BinaryType::Dec)
                    } else if nc == '.' {
                        Some(BinaryType::Unknown)
                    } else {
                        None
                    }
                }
                '0' => match nc {
                    'X' | 'x' => Some(BinaryType::Hex),
                    '0'..'7' => Some(BinaryType::Oct),
                    _ => {
                        if is_separator(nc) {
                            Some(BinaryType::Dec)
                        } else {
                            None
                        }
                    }
                },
                _ => None,
            },
        }
    }
}

pub fn is_reserved_word(word: String) -> bool {
    matches!(
        word.as_str(),
        "if" | "else"
            | "while"
            | "for"
            | "return"
            | "break"
            | "continue"
            | "int"
            | "float"
            | "double"
            | "char"
    )
}

pub fn is_bool_identifier(word: String) -> bool {
    matches!(word.as_str(), "true" | "false")
}

pub fn is_comment(c: char, nc: char) -> bool {
    let is_single_line_comment: bool = c == '/' && nc == '/';
    let is_multi_line_comment: bool = c == '/' && nc == '*';
    is_multi_line_comment | is_single_line_comment
}
