use crate::ast::types::ASTNode;
use crate::ir::irgen::IrType;
use crate::lexer::lexer::*;
use crate::parser::parse::CSTNode;

use super::types::PhraseType;

pub fn print_cst(node: &Vec<CSTNode>) {
    println!("{:?}", node.len());
    for val in node {
        val.print_tree();
        // println!("{:?}", val);
    }
}
pub fn print_ast(node: &Vec<ASTNode>) {
    println!("{:?}", node.len());
    for val in node {
        // val.print_tree();
        println!("{}", val);
    }
}

pub fn print_tokens(tokens: &Vec<Token>) {
    for token in tokens {
        println!("{:?} {:?}", token.types, token.value);
    }
}

pub fn print_ir(tokens: &Vec<IrType>) {
    for token in tokens {
        println!("{:?}", token);
    }
}

pub fn print_err_info(err_info: Vec<String>) {
    for info in err_info {
        println!("{:?}", info);
    }
}

pub fn symbol_is_literal(symbol: &PhraseType) -> bool {
    match symbol {
        PhraseType::Identifier => false,
        PhraseType::Bool
        | PhraseType::String
        | PhraseType::Hex
        | PhraseType::Oct
        | PhraseType::Dec
        | PhraseType::Float
        | PhraseType::Char => true,
        _ => unreachable!(),
    }
}
