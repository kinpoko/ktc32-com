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
#[derive(Debug)]
struct TokenList(Vec<Token>);

impl TokenList {
    pub fn iter(&self) -> std::slice::Iter<Token> {
        self.0.iter()
    }

    pub fn consume(&self, iter: &mut std::slice::Iter<Token>, op: char) -> bool {
        let token = iter.clone().next().unwrap();
        if token.kind != TokenKind::Reserved || token.str.chars().next().unwrap() != op {
            return false;
        }
        iter.next();
        true
    }
    pub fn expect(&self, iter: &mut std::slice::Iter<Token>, op: char) -> Result<()> {
        let token = iter.clone().next().unwrap();
        if token.kind != TokenKind::Reserved || token.str.chars().next().unwrap() != op {
            return Err(anyhow!(" It is not {}", op));
        }
        iter.next();
        Ok(())
    }
    pub fn expect_number(&self, iter: &mut std::slice::Iter<Token>) -> Result<i64> {
        let token = iter.clone().next().unwrap();
        if token.kind != TokenKind::Num {
            return Err(anyhow!(" It is not number {}", token.str));
        }
        iter.next();
        Ok(token.val)
    }
    pub fn at_eof(&self, iter: &mut std::slice::Iter<Token>) -> bool {
        let token = iter.clone().next().unwrap();
        token.kind == TokenKind::Eof
    }
}

fn tokenize(mut p: String) -> Result<TokenList> {
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
    Ok(TokenList(token_list))
}

fn main() -> Result<()> {
    if env::args().len() != 2 {
        return Err(anyhow!("Incorrect number of arguments"));
    }
    let p = env::args().nth(1).unwrap();

    let token_list = tokenize(p)?;
    let mut iter = token_list.iter();

    println!("main:");
    println!("  addi a0, r0, {}", token_list.expect_number(&mut iter)?);

    while !token_list.at_eof(&mut iter) {
        if token_list.consume(&mut iter, '+') {
            println!("  addi a0, a0, {}", token_list.expect_number(&mut iter)?);
            continue;
        }

        token_list.expect(&mut iter, '-')?;
        println!("  addi a0, a0, -{}", token_list.expect_number(&mut iter)?);
    }
    println!("  jalr r0, r0, 12");

    Ok(())
}
