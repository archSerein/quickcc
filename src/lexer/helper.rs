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
    Bool,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WordType {
    Identifier,
    Keyword,
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
                let next_state = WordType::word_type(c, next_c, WordType::Operator);
                if next_c as u8 == SPACE || is_separator(next_c) {
                    Some(State::Accepted(next_state))
                } else {
                    Some(State::Handling(next_state))
                }
            } else if is_valid_char(c) {
                let next_c = nc.unwrap_or(' ');
                let next_type = WordType::word_type(c, next_c, WordType::Unknown);
                if next_c as u8 == SPACE || is_separator(next_c) {
                    Some(State::Accepted(next_type))
                } else {
                    Some(State::Handling(next_type))
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
                Some(State::Accepted(current_type))
            } else {
                let next_type = WordType::word_type(c, next_c, current_type);
                if current_type != WordType::Unknown && next_type == WordType::Unknown {
                    Some(State::Unaccepted)
                } else {
                    Some(State::Handling(next_type))
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
    matches!(c, '(' | ')' | '{' | '}' | '[' | ']' | ';' | ',' | ':' | '.')
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
    pub fn literal_type(c: char, nc: char, state: LiteralType) -> Option<LiteralType> {
        match state {
            LiteralType::Integer(ref t) => {
                if let Some(next_state) = BinaryType::binary_type(c, nc, *t) {
                    Some(LiteralType::Integer(next_state))
                } else {
                    None
                }
            }
            LiteralType::Float => match c {
                '0'..='9' => Some(LiteralType::Float),
                _ => None,
            },
            LiteralType::Char => {
                if c.is_ascii() && nc as u8 == 0x27 {
                    Some(LiteralType::Char)
                } else {
                    None
                }
            }
            LiteralType::Bool => match c {
                't' | 'r' | 'u' | 'e' | 'f' | 'a' | 'l' | 's' => Some(LiteralType::Bool),
                _ => None,
            },
            LiteralType::Unknown => match c {
                '0'..='9' => {
                    if let Some(t) = BinaryType::binary_type(c, nc, BinaryType::Unknown) {
                        Some(LiteralType::Integer(t))
                    } else {
                        Some(LiteralType::Integer(BinaryType::Dec))
                    }
                }
                '\'' => Some(LiteralType::Char),
                't' | 'r' | 'u' | 'e' | 'f' | 'a' | 'l' | 's' => Some(LiteralType::Bool),
                _ => None,
            },
        }
    }
}

impl WordType {
    pub fn word_type(c: char, nc: char, state: WordType) -> WordType {
        match state {
            WordType::Unknown => match c {
                'a'..='z' | 'A'..='Z' | '_' => WordType::Identifier,
                '0'..='9' => {
                    if let Some(t) = LiteralType::literal_type(c, nc, LiteralType::Unknown) {
                        WordType::Literal(t)
                    } else {
                        WordType::Unknown
                    }
                }
                _ => WordType::Unknown,
            },
            WordType::Identifier => match c {
                'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => WordType::Identifier,
                _ => WordType::Unknown,
            },
            WordType::Literal(ref t) => {
                if let Some(next_state) = LiteralType::literal_type(c, nc, *t) {
                    WordType::Literal(next_state)
                } else {
                    WordType::Unknown
                }
            }
            WordType::String => WordType::String,
            WordType::Comment => WordType::Comment,
            WordType::Operator => {
                if c == '/' && nc == '/' {
                    WordType::Comment
                } else {
                    WordType::Operator
                }
            }
            WordType::Separator => WordType::Separator,
            WordType::Keyword => WordType::Keyword,
        }
    }
}

impl BinaryType {
    pub fn binary_type(c: char, nc: char, state: BinaryType) -> Option<BinaryType> {
        match c {
            'x' => Some(BinaryType::Hex),
            _ => None,
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
            | "main"
    )
}
