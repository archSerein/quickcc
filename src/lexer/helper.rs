#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryType {
    Hex,
    Oct,
    Dec,
    Bin,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LiteralType {
    Integer(BinaryType),
    Float,
    Char,
    Bool,
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

pub fn translation(c: char, nc: Option<char>, state: State) -> Option<State> {
    match state {
        State::Init => {
            if is_invisible_char(c as u8) {
                Some(State::Init)
            } else {
                Some(State::Handling(WordType::Unknown))
            }
        }

        State::Handling(current_type) => {
            // 安全地处理 Option<char>
            let next_c = nc.unwrap_or(' '); // 如果 None，则默认 ' '
            let next_type = WordType::word_type(c, next_c, current_type);
            Some(State::Handling(next_type))
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

impl LiteralType {
    pub fn literal_type(c: char, nc: char) -> Option<LiteralType> {
        if c == '0' {
            match nc {
                'x' => Some(LiteralType::Integer(BinaryType::Hex)),
                'b' => Some(LiteralType::Integer(BinaryType::Bin)),
                _ if nc > '0' && nc < '8' => Some(LiteralType::Integer(BinaryType::Oct)),
                _ if is_invisible_char(nc as u8) => Some(LiteralType::Integer(BinaryType::Dec)),
                _ => None,
            }
        } else if c > '0' && c <= '9' {
            if nc >= '0' && nc <= '9' {
                Some(LiteralType::Integer(BinaryType::Dec))
            } else if nc == '.' {
                Some(LiteralType::Float)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl WordType {
    pub fn word_type(c: char, nc: char, state: WordType) -> WordType {
        match state {
            WordType::Unknown => match c {
                'a'..='z' | 'A'..='Z' | '_' => WordType::Identifier,
                '0'..='9' => {
                    if let Some(t) = LiteralType::literal_type(c, nc) {
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
            WordType::Literal(_) => match c {
                '0'..='9' => WordType::Literal(LiteralType::Integer(BinaryType::Dec)),
                '.' => WordType::Literal(LiteralType::Float),
                _ => WordType::Unknown,
            },
            WordType::String => WordType::String,
            WordType::Comment => WordType::Comment,
            WordType::Operator => WordType::Operator,
            WordType::Separator => WordType::Separator,
            WordType::Keyword => WordType::Keyword,
        }
    }
}

impl BinaryType {
    pub fn binary_type(c: char) -> Option<BinaryType> {
        match c {
            'x' => Some(BinaryType::Hex),
            'b' => Some(BinaryType::Bin),
            _ => None,
        }
    }
}

pub fn is_reserved_word(word: String) -> bool {
    matches!(
        word.as_str(),
        "if" | "else" | "while" | "for" | "return" | "break" | "continue"
    )
}
