#[derive(Debug, Clone, PartialEq)]
pub enum PhraseType {
    Hex,
    Dec,
    Oct,
    Keyword,
    Bool,
    Identifier,
    Operator,
    Separator,
    Unknown,
    String,
    Float,
    Char,
}
