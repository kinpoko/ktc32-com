mod strtol;
use anyhow::{anyhow, Ok, Result};
use std::env;
use strtol::strtol;

fn main() -> Result<()> {
    if env::args().len() != 2 {
        return Err(anyhow!("Incorrect number of arguments"));
    }
    let p = env::args().nth(1).unwrap();

    println!("main:");
    let (mut p, n) = strtol(p);
    println!("  addi a0, r0, {}", n.unwrap());

    while let Some(c) = p.chars().next() {
        if c == '+' {
            p = p.split_off(1);
            let (r, n) = strtol(p);
            p = r;
            println!("  addi a0, r0, {}", n.unwrap());
            continue;
        }
        if c == '-' {
            p = p.split_off(1);
            let (r, n) = strtol(p);
            p = r;
            println!("  addi a0, r0, -{}", n.unwrap());
            continue;
        }

        return Err(anyhow!("unexpected charactor {}", c));
    }

    println!("  jalr r0, r0, 12");
    Ok(())
}
