use anyhow::{anyhow, Ok, Result};
use std::env;

fn main() -> Result<()> {
    if env::args().len() != 3 {
        return Err(anyhow!("Incorrect number of arguments"));
    }
    let args: Vec<String> = env::args().collect();
    println!("main:");
    println!("  addi ra, r0, {}", args[2]);
    println!("  jalr r0, r0, 12");
    Ok(())
}
