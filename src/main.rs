mod codegen;
mod parse;
mod token;
use anyhow::{anyhow, Ok, Result};
use codegen::gen;
use parse::expr;
use std::env;
use token::tokenize;

fn main() -> Result<()> {
    if env::args().len() != 2 {
        return Err(anyhow!("Incorrect number of arguments"));
    }
    let p = env::args().nth(1).unwrap();

    let token_list = tokenize(p)?;

    let mut i: usize = 0;
    let node = expr(&token_list, &mut i)?;

    println!("main:");
    gen(node);
    println!("  lw a0, sp, 0");
    println!("  jalr r0, r0, 12");

    Ok(())
}
