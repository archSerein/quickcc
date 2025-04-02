use crate::Node;
use crate::lexer::lexer::*;

pub fn print_ast(root: Node) {
    todo!()
}

pub fn print_tokens(tokens: &Vec<Token>) {
    for token in tokens {
        println!("{:?} {:?}", token.token_type, token.value);
    }
}

pub fn print_err_info(err_info: Vec<String>) {
    for info in err_info {
        println!("{:?}", info);
    }
}
