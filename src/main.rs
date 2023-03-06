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

fn consume(token: &Token, op: &str) -> bool {
    if token.kind != TokenKind::Reserved || token.str != op {
        return false;
    }
    true
}

fn expect(token: &Token, op: &str) -> Result<()> {
    if token.kind != TokenKind::Reserved || token.str != op {
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

        if c == '+' || c == '-' || c == '*' || c == '/' || c == '(' || c == ')' {
            p = p.split_off(1);
            token_list.push(Token {
                kind: TokenKind::Reserved,
                val: 0,
                str: c.to_string(),
            });
            continue;
        }

        if c == '=' || c == '!' {
            p = p.split_off(1);
            let h = p.chars().next().unwrap();
            if h == '=' {
                token_list.push(Token {
                    kind: TokenKind::Reserved,
                    val: 0,
                    str: c.to_string() + &h.to_string(),
                });
                p = p.split_off(1);
                continue;
            } else {
                return Err(anyhow!("could not tokenize {}", h));
            }
        }

        if c == '>' || c == '<' {
            p = p.split_off(1);
            let h = p.chars().peekable().peek().cloned().unwrap();
            if h == '=' {
                token_list.push(Token {
                    kind: TokenKind::Reserved,
                    val: 0,
                    str: c.to_string() + &h.to_string(),
                });
                p = p.split_off(1);
                continue;
            } else {
                token_list.push(Token {
                    kind: TokenKind::Reserved,
                    val: 0,
                    str: c.to_string(),
                });
                continue;
            }
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

#[derive(Debug, PartialEq)]
enum NodeKind {
    Add,
    Sub,
    Mul,
    Div,
    Num,
}
#[derive(Debug)]
struct Node {
    kind: NodeKind,
    lhs: Option<Box<Node>>,
    rhs: Option<Box<Node>>,
    val: i64,
}

fn new_node(kind: NodeKind, lhs: Node, rhs: Node) -> Node {
    Node {
        kind,
        lhs: Some(Box::new(lhs)),
        rhs: Some(Box::new(rhs)),
        val: 0,
    }
}

fn new_node_num(val: i64) -> Node {
    Node {
        kind: NodeKind::Num,
        lhs: None,
        rhs: None,
        val,
    }
}

fn expr(token_list: &Vec<Token>, i: &mut usize) -> Result<Node> {
    let mut node = mul(token_list, i)?;
    loop {
        if consume(&token_list[*i], "+") {
            *i += 1;
            node = new_node(NodeKind::Add, node, mul(token_list, i)?);
        } else if consume(&token_list[*i], "-") {
            *i += 1;
            node = new_node(NodeKind::Sub, node, mul(token_list, i)?);
        } else {
            return Ok(node);
        }
    }
}

fn mul(token_list: &Vec<Token>, i: &mut usize) -> Result<Node> {
    let mut node = unary(token_list, i)?;

    loop {
        if consume(&token_list[*i], "*") {
            *i += 1;
            node = new_node(NodeKind::Mul, node, mul(token_list, i)?);
        } else if consume(&token_list[*i], "/") {
            *i += 1;
            node = new_node(NodeKind::Div, node, mul(token_list, i)?);
        } else {
            return Ok(node);
        }
    }
}

fn unary(token_list: &Vec<Token>, i: &mut usize) -> Result<Node> {
    if consume(&token_list[*i], "+") {
        *i += 1;
        return primary(token_list, i);
    }
    if consume(&token_list[*i], "-") {
        *i += 1;
        return Ok(new_node(
            NodeKind::Sub,
            new_node_num(0),
            primary(token_list, i)?,
        ));
    }
    primary(token_list, i)
}

fn primary(token_list: &Vec<Token>, i: &mut usize) -> Result<Node> {
    if consume(&token_list[*i], "(") {
        *i += 1;
        let node = expr(token_list, i)?;
        expect(&token_list[*i], ")")?;
        *i += 1;
        return Ok(node);
    }
    *i += 1;
    Ok(new_node_num(expect_number(&token_list[*i - 1])?))
}

fn gen(node: Node) {
    if node.kind == NodeKind::Num {
        println!("  addi t0, r0, {}", node.val);
        println!("  addi sp, sp, -4");
        println!("  sw t0, sp, 0",);
        return;
    }

    gen(*node.lhs.unwrap());
    gen(*node.rhs.unwrap());

    println!("  lw a1, sp, 0");
    println!("  addi sp, sp, 4");
    println!("  lw a0, sp, 0");
    println!("  addi sp, sp, 4");

    match node.kind {
        NodeKind::Add => {
            println!("  add a0, a1");
        }
        NodeKind::Sub => {
            println!("  sub a0, a1");
        }
        NodeKind::Mul => {
            println!("  mov t0, a0");
            println!("  addi a1, a1, -1");
            println!("  beq a1, zero, 6");
            println!("  add a0, t0");
            println!("  jal zero, -14");
        }
        NodeKind::Div => {
            println!("  mov t0, zero");
            println!("  blt a0, a1, 10");
            println!("  addi t0, t0, 1");
            println!("  sub a0, a1");
            println!("  jal zero, -14");
            println!("  mov a0, t0");
        }

        _ => {}
    }
    println!("  addi sp, sp, -4");
    println!("  sw a0, sp, 0",);
}

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
