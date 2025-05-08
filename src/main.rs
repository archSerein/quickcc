use std::env;

// 词法分析器模块
mod lexer;
mod parser;
mod utils;
use parser::parse::parse;
use utils::helper::print_cst;
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
    print_tokens(&lexer_tokens);
    let cst_tokens = parse(&lexer_tokens);
    print_cst(&cst_tokens);
    // let ir_tokens
    // let asm_tokens
}
