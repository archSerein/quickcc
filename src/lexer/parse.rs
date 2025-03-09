use lexer::dfa;
use lexer::file::Source;
use lexer::token::{Token, Word};

use super::dfa::INIT;

// 执行词法分析器parse
pub fn run(filepath: &str) -> Vec<Token> {
    let mut f = Source::new(filepath);
    // 识别相应词法单元的函数，由choose_dfa生成相应的识别函数
    let mut dfa: Option<dfa::DFA> = None;
    // 当一个词素识别完成后，这是重新生成dfa识别函数的的标志
    let mut change_dfa: bool = true;
    // 当前状态
    let mut state: dfa::State = dfa::State::MoveTo(INIT);
    // 标记单词在文本中实际的位置
    let mut start_row: u32 = 1;
    let mut start_col: u32 = 1;
    // 词素开始的位置
    let mut start: usize = 0;
    // 词素结束的位置
    let mut end: usize = 0;
    // 匹配到的单词
    let mut word: String;
    // 用于存储Token的Vector
    let mut tokens: Vec<Token> = Vec::new();

    while let Some(c) = f.get_char() {
        if change_dfa {
            if let Some(nc) = f.look_forward() {
                dfa = dfa::choose_dfa(c, nc);
            } else {
                dfa = dfa::choose_dfa(c, ' ');
            }
            start = f.get_pointer();
            start_row = f.position().0;
            start_col = f.position().1;
        }
        match state {
            dfa::State::Accepted(ref t) => {
                f.back_pointer();
                end = f.get_pointer();
                word = f.get_word(start, end);
                match *t {
                    dfa::WordType::Id => {
                        if Token::is_keyword(&word) {
                            tokens.push(Token::new(
                                Word::Keyword(Word::get_index("kw", word)),
                                start_row,
                                start_col,
                            ));
                        } else {
                            tokens.push(Token::new(Word::Id(word), start_row, start_col));
                        }
                    }
                    dfa::WordType::Operator => {
                        tokens.push(Token::new(
                            Word::Operator(Word::get_index("op", word)),
                            start_row,
                            start_col,
                        ));
                    }
                    dfa::WordType::Separator => {
                        tokens.push(Token::new(
                            Word::Separator(Word::get_index("sp", word)),
                            start_row,
                            start_col,
                        ));
                    }
                    dfa::WordType::Value => {
                        tokens.push(Token::new(Word::Value(word), start_row, start_col));
                    }
                    _ => (),
                }
                change_dfa = true;
            }
            dfa::State::Unaccepted => {
                println!(
                    "Unaccepted -> position row {}, col {}",
                    start_row, start_col
                );
                panic!("Unhandled error");
            }
            dfa::State::MoveTo(s) => {
                change_dfa = false;
                if let Some(func) = &dfa {
                    state = func(s, c);
                } else {
                    panic!("DFA function is None");
                }
                if (c as u8) == Source::newline() {
                    f.add_row();
                    f.add_col();
                } else {
                    f.add_col();
                }
                f.next_pointer();
            }
        }
    }
    tokens
}
