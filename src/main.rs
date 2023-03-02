mod strtol;
use anyhow::{anyhow, Ok, Result};
use std::env;
use strtol::strtol;

#[derive(Debug, PartialEq)]
enum TokenKind {
    Reserved,
    Num,
    Eof,
}

#[derive(Debug)]
struct Token {
    kind: TokenKind,
    val: i64,
    str: String,
}

fn consume(token: &Token, op: char) -> bool {
    if token.kind != TokenKind::Reserved || token.str.chars().next().unwrap() != op {
        return false;
    }
    true
}

fn expect(token: &Token, op: char) -> Result<()> {
    if token.kind != TokenKind::Reserved || token.str.chars().next().unwrap() != op {
        return Err(anyhow!(" It is not {}", op));
    }
    Ok(())
}

fn expect_number(token: &Token) -> Result<i64> {
    if token.kind != TokenKind::Num {
        return Err(anyhow!(" It is not number {}", token.str));
    }
    Ok(token.val)
}

fn at_eof(token: &Token) -> bool {
    token.kind == TokenKind::Eof
}

fn tokenize(mut p: String) -> Result<Vec<Token>> {
    let mut token_list: Vec<Token> = Vec::new();
    while let Some(c) = p.chars().next() {
        if c.is_whitespace() {
            p = p.split_off(1);
            continue;
        }

        if c == '+' || c == '-' {
            p = p.split_off(1);
            token_list.push(Token {
                kind: TokenKind::Reserved,
                val: 0,
                str: c.to_string(),
            });
            continue;
        }

        if c.is_ascii_digit() {
            let (r, n) = strtol(p);
            p = r;
            token_list.push(Token {
                kind: TokenKind::Num,
                val: n.unwrap(),
                str: c.to_string(),
            });
            continue;
        }
        return Err(anyhow!("could not tokenize {}", c));
    }

    token_list.push(Token {
        kind: TokenKind::Eof,
        val: 0,
        str: "".to_string(),
    });
    Ok(token_list)
}

fn main() -> Result<()> {
    if env::args().len() != 2 {
        return Err(anyhow!("Incorrect number of arguments"));
    }
    let p = env::args().nth(1).unwrap();

    let token_list = tokenize(p)?;

    println!("main:");
    println!("  addi a0, r0, {}", expect_number(&token_list[0])?);

    let mut i = 1;
    while i <= token_list.len() {
        if at_eof(&token_list[i]) {
            break;
        }

        if consume(&token_list[i], '+') {
            i += 1;
            println!("  addi a0, a0, {}", expect_number(&token_list[i])?);
            i += 1;
            continue;
        }

        expect(&token_list[i], '-')?;
        i += 1;
        println!("  addi a0, a0, -{}", expect_number(&token_list[i])?);
        i += 1;
    }
    println!("  jalr r0, r0, 12");

    Ok(())
}
