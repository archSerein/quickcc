use std::env;

// 词法分析器模块
mod asm;
mod ast;
mod ir;
mod lexer;
mod parser;
mod utils;
use asm::write_asm::write_asm;
use ast::astgen::ast_gen;
use ir::irgen::ir_gen;
use parser::parse::parse;
use utils::helper::print_ast;
use utils::helper::print_cst;
use utils::helper::print_ir;
use utils::helper::print_tokens;

// mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please input the filename!");
    }
    let filename = match args.get(1) {
        Some(name) => name,
        None => panic!("Get filename error!"),
    };

    let lexer_tokens = lexer::lexer::run(&filename[..]);
    // print_tokens(&lexer_tokens);
    let cst = parse(&lexer_tokens);
    // print_cst(&cst);
    let ast = ast_gen(&cst);
    print_ast(&ast);
    let ir = ir_gen(&ast);
    print_ir(&ir);
    write_asm(&ir, &filename);
}
