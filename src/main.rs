mod codegen;
mod parse;
mod token;
use codegen::CodeGenerator;
use parse::Parser;
use std::env;
use token::tokenize;

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of arguments");
    }
    let p = env::args().nth(1).unwrap();

    let token_list = tokenize(p);
    // println!("{:?}", token_list);

    let mut parser = Parser::new(token_list);
    let node_list = parser.program();

    let codegen = CodeGenerator::new(node_list);

    println!("{:?}", codegen.node_list);

    println!("main:");
    println!("  addi sp, sp, -8");
    println!("  sw ra, sp, 4");
    println!("  sw fp, sp, 0");
    println!("  mov fp, sp");
    println!("  addi sp, sp -{}", parser.locals.offset);

    codegen.gen_code();
}
