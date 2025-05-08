use crate::lexer::lexer::*;
use crate::parser::parse::CSTNode;

pub fn print_cst(node: &Vec<CSTNode>) {
    println!("{:?}", node.len());
    for val in node {
        println!("{:?}", val);
    }
}

pub fn print_tokens(tokens: &Vec<Token>) {
    for token in tokens {
        println!("{:?} {:?}", token.types, token.value);
    }
}

pub fn print_err_info(err_info: Vec<String>) {
    for info in err_info {
        println!("{:?}", info);
    }
}
