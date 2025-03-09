use std::env;
// 词法分析器模块
mod lexer;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please input the filename!");
    }
    let filename = match args.get(1) {
        Some(name) => name,
        None => panic!("Get filename error!"),
    };

    let lexer_tokens = lexer::parse::run(&filename[..]);
    // let ast_tokens =
    // let ir_tokens
    // let asm_tokens
    // let obj_tokens
    // let elf_tokens
}
