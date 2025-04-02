use crate::lexer::lexer::Token;

pub struct Node {
    symbol: String,
    child: Vec<Node>,
}

pub fn parse(tokens: &Vec<Token>) -> Node {
    todo!()
}
