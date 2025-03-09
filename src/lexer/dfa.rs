use lexer::file::Source;

// pub type DFA = fn(u8, char) -> State;
pub type DFA = Box<dyn Fn(u8, char) -> State>;

pub enum WordType {
    Id,
    Value,
    Operator,
    Separator,
    Other,
}

pub const INIT: u8 = 0;
pub const HANDLE: u8 = 1;
pub const DONE: u8 = 2;

pub enum State {
    MoveTo(u8),         // 待转移状态
    Unaccepted,         // 不可接受状态
    Accepted(WordType), // 可接受状态
}

pub fn choose_dfa(c: char, nc: char) -> Option<DFA> {
    match c {
        _ if c.is_alphabetic() => Some(Box::new(dfa_id)),
        _ if Source::is_invisible_char(c as u8) => Some(Box::new(dfa_whitespace)),
        _ if c.is_numeric() => Some(Box::new(dfa_num)),
        _ if c == '\'' => Some(Box::new(dfa_char)),
        _ if c == '"' => Some(Box::new(dfa_string)),
        _ if c == '/' && (nc == '*' || nc == '/') => Some(Box::new(dfa_comments)),
        _ if c == '{' || c == '}' || c == '(' || c == ')' || c == ';' || c == ',' => {
            Some(Box::new(dfa_separator))
        }
        _ => Some(Box::new(dfa_operator)),
    }
}

// 识别标识符的DFA
pub fn dfa_id(s: u8, c: char) -> State {
    match s {
        INIT => {
            if c == '_' || c.is_alphabetic() {
                State::MoveTo(DONE)
            } else {
                State::Unaccepted
            }
        }
        DONE => {
            if c == '_' || c.is_alphabetic() || c.is_numeric() {
                State::MoveTo(INIT)
            } else {
                State::Accepted(WordType::Id)
            }
        }
        _ => State::Unaccepted,
    }
}

// 匹配空白符的dfa
pub fn dfa_whitespace(s: u8, c: char) -> State {
    match s {
        0 => {
            if Source::is_invisible_char(c as u8) {
                State::MoveTo(1)
            } else {
                State::Unaccepted
            }
        }
        1 => {
            if Source::is_invisible_char(c as u8) {
                State::MoveTo(1)
            } else {
                State::Accepted(WordType::Other)
            }
        }
        _ => State::Unaccepted,
    }
}

// 匹配数字(整形和浮点型)的dfa
pub fn dfa_num(s: u8, c: char) -> State {
    match s {
        0 => {
            if c.is_numeric() {
                State::MoveTo(1)
            } else {
                State::Unaccepted
            }
        }
        1 => {
            if c.is_numeric() {
                State::MoveTo(1)
            } else if c == '.' {
                State::MoveTo(2)
            } else {
                State::Accepted(WordType::Value)
            }
        }
        2 => {
            if c.is_numeric() {
                State::MoveTo(3)
            } else {
                State::Unaccepted
            }
        }
        3 => {
            if c.is_numeric() {
                State::MoveTo(3)
            } else {
                State::Accepted(WordType::Value)
            }
        }
        _ => State::Unaccepted,
    }
}

// 匹配运算符的DFA
pub fn dfa_operator(s: u8, c: char) -> State {
    match s {
        0 => match c {
            '<' | '>' | '=' | '!' | '+' | '-' | '*' | '/' => State::MoveTo(1),
            _ => State::Unaccepted,
        },
        1 => {
            if c == '=' {
                State::MoveTo(2)
            } else {
                State::Accepted(WordType::Operator)
            }
        }
        2 => State::Accepted(WordType::Operator),
        _ => State::Unaccepted,
    }
}

// 匹配值类型中的字符值
pub fn dfa_char(s: u8, c: char) -> State {
    match s {
        0 => {
            if c == '\'' {
                State::MoveTo(1)
            } else {
                State::Unaccepted
            }
        }
        1 => {
            // ASCII字符从32（空格）到126（~）
            if (c as u8) >= 32 && (c as u8) <= 126 {
                State::MoveTo(2)
            } else {
                State::Unaccepted
            }
        }
        2 => {
            if c == '\'' {
                State::MoveTo(3)
            } else {
                State::Unaccepted
            }
        }
        3 => State::Accepted(WordType::Value),
        _ => State::Unaccepted,
    }
}

// 匹配分隔符的DFA
pub fn dfa_separator(s: u8, c: char) -> State {
    match s {
        0 => match c {
            '{' | '}' | '(' | ')' | ',' | ';' => State::MoveTo(1),
            _ => State::Unaccepted,
        },
        1 => State::Accepted(WordType::Separator),
        _ => State::Unaccepted,
    }
}

// 匹配值类型中的字符串
fn dfa_string(s: u8, c: char) -> State {
    match s {
        0 => {
            if c == '"' {
                State::MoveTo(1)
            } else {
                State::Unaccepted
            }
        }
        1 => {
            if c != '"' {
                State::MoveTo(1)
            } else {
                State::MoveTo(2)
            }
        }
        2 => State::Accepted(WordType::Value),
        _ => State::Unaccepted,
    }
}

// 匹配注释的DFA
pub fn dfa_comments(s: u8, c: char) -> State {
    match s {
        0 => {
            if c == '/' {
                State::MoveTo(1)
            } else {
                State::Unaccepted
            }
        }
        1 => {
            if c == '*' {
                State::MoveTo(2)
            } else if c == '/' {
                State::MoveTo(3)
            } else {
                State::Unaccepted
            }
        }
        2 => {
            if c == '*' {
                State::MoveTo(4)
            } else {
                State::MoveTo(5)
            }
        }
        3 => {
            if (c as u8) == Source::newline() {
                State::MoveTo(7)
            } else {
                State::MoveTo(6)
            }
        }
        4 => {
            if c == '/' {
                State::MoveTo(7)
            } else {
                State::Unaccepted
            }
        }
        5 => {
            if c == '*' {
                State::MoveTo(4)
            } else {
                State::MoveTo(5)
            }
        }
        6 => {
            if (c as u8) == Source::newline() {
                State::MoveTo(7)
            } else {
                State::MoveTo(6)
            }
        }
        7 => State::Accepted(WordType::Other),
        _ => State::Unaccepted,
    }
}
