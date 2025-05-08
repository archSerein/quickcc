use super::types::State;
use crate::lexer::lexer::Token;
use crate::utils::types::PhraseType;

pub fn look_forward(tokens: &Vec<Token>, start: usize, len: usize) -> Vec<Token> {
    if start >= tokens.len() {
        vec![]
    } else {
        let end = usize::min(start + len, tokens.len());
        tokens[start..end].to_vec()
    }
}

pub fn error_handler(state: &Vec<State>, look: &Token) {
    // 分析异常原因
    println!(
        "current state {:?} unexpected {:?}",
        state.last().unwrap(),
        look.value
    );
}

pub fn is_type_keyword(value: &String) -> bool {
    matches!(
        value.as_str(),
        "int" | "uint" | "float" | "double" | "char" | "String"
    )
}

pub fn is_control_keyword(value: &String) -> bool {
    matches!(
        value.as_str(),
        "if" | "else" | "while" | "for" | "return" | "break" | "continue"
    )
}

/// 把 token_type 映射到 action_table 的列号
pub fn term_index(tok: &Token) -> usize {
    use PhraseType::*;
    match tok.types {
        Operator if tok.value == "!" => 0,
        Operator if tok.value == "!=" => 1,
        Operator if tok.value == "&&" => 2,
        Separator if tok.value == "(" => 3,
        Separator if tok.value == ")" => 4,
        Operator if tok.value == "*" => 5,
        Operator if tok.value == "+" => 6,
        Separator if tok.value == "," => 7,
        Operator if tok.value == "-" => 8,
        Operator if tok.value == "/" => 9,
        Separator if tok.value == ";" => 10,
        Operator if tok.value == "<" => 11,
        Operator if tok.value == "<=" => 12,
        Operator if tok.value == "=" => 13,
        Operator if tok.value == "==" => 14,
        Operator if tok.value == ">" => 15,
        Operator if tok.value == ">=" => 16,
        Keyword if is_type_keyword(&tok.value) => 17,
        Separator if tok.value == "[" => 18,
        Separator if tok.value == "]" => 19,
        Keyword if tok.value == "else" => 20,
        Identifier if tok.value == "false" => 21,
        Identifier if tok.value != "true" => 22,
        Keyword if tok.value == "if" => 23,
        Float | Bool | Char | Hex | Dec | Oct => 24,
        Keyword if tok.value == "return" => 25,
        Keyword if tok.value == "struct" => 26,
        Identifier if tok.value == "true" => 27,
        Keyword if tok.value == "while" => 28,
        Separator if tok.value == "{" => 29,
        Operator if tok.value == "||" => 30,
        Separator if tok.value == "32" => 31,
        _ => 32, // end of file
    }
}
