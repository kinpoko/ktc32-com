mod codegen;
mod parse;
mod token;
use codegen::gen;
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

    // println!("{:?}", node_list);

    println!("main:");
    println!("  addi sp, sp, -8");
    println!("  sw ra, sp, 4");
    println!("  sw fp, sp, 0");
    println!("  mov fp, sp");
    println!("  addi sp, sp -{}", parser.locals.offset);

    for node in node_list {
        gen(node);
        println!("  addi sp, sp, 4");
    }
}
