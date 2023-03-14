use crate::token::{Token, TokenKind};
use anyhow::{anyhow, Ok, Result};

fn consume(token: &Token, op: &str) -> bool {
    if token.kind != TokenKind::Reserved || token.str != op {
        return false;
    }
    true
}

fn consume_ident(token: &Token) -> bool {
    if token.kind != TokenKind::Ident {
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

#[derive(Debug, PartialEq)]
pub enum NodeKind {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Ne,
    Lt,
    Le,
    Assign,
    Lvar,
    Num,
}
#[derive(Debug)]
pub struct Node {
    pub kind: NodeKind,
    pub lhs: Option<Box<Node>>,
    pub rhs: Option<Box<Node>>,
    pub val: i64,
    pub offset: i64,
}

fn new_node(kind: NodeKind, lhs: Node, rhs: Node) -> Node {
    Node {
        kind,
        lhs: Some(Box::new(lhs)),
        rhs: Some(Box::new(rhs)),
        val: 0,
        offset: 0,
    }
}

fn new_node_lvar(offset: i64) -> Node {
    Node {
        kind: NodeKind::Lvar,
        lhs: None,
        rhs: None,
        val: 0,
        offset,
    }
}

fn new_node_num(val: i64) -> Node {
    Node {
        kind: NodeKind::Num,
        lhs: None,
        rhs: None,
        val,
        offset: 0,
    }
}

pub fn program(token_list: &Vec<Token>, i: &mut usize) -> Result<Vec<Node>> {
    let mut code: Vec<Node> = Vec::new();
    while !at_eof(&token_list[*i]) {
        code.push(stmt(token_list, i)?);
    }
    Ok(code)
}

fn stmt(token_list: &Vec<Token>, i: &mut usize) -> Result<Node> {
    let node = expr(token_list, i)?;
    expect(&token_list[*i], ";")?;
    *i += 1;
    Ok(node)
}

fn expr(token_list: &Vec<Token>, i: &mut usize) -> Result<Node> {
    assign(token_list, i)
}

fn assign(token_list: &Vec<Token>, i: &mut usize) -> Result<Node> {
    let mut node = equality(token_list, i)?;
    if consume(&token_list[*i], "=") {
        *i += 1;
        node = new_node(NodeKind::Assign, node, assign(token_list, i)?);
    }
    Ok(node)
}

fn equality(token_list: &Vec<Token>, i: &mut usize) -> Result<Node> {
    let mut node = relational(token_list, i)?;
    loop {
        if consume(&token_list[*i], "==") {
            *i += 1;
            node = new_node(NodeKind::Eq, node, relational(token_list, i)?);
        } else if consume(&token_list[*i], "!=") {
            *i += 1;
            node = new_node(NodeKind::Ne, node, relational(token_list, i)?);
        } else {
            return Ok(node);
        }
    }
}

fn relational(token_list: &Vec<Token>, i: &mut usize) -> Result<Node> {
    let mut node = add(token_list, i)?;
    loop {
        if consume(&token_list[*i], "<") {
            *i += 1;
            node = new_node(NodeKind::Lt, node, add(token_list, i)?);
        } else if consume(&token_list[*i], "<=") {
            *i += 1;
            node = new_node(NodeKind::Le, node, add(token_list, i)?);
        } else if consume(&token_list[*i], ">") {
            *i += 1;
            node = new_node(NodeKind::Lt, add(token_list, i)?, node);
        } else if consume(&token_list[*i], ">=") {
            *i += 1;
            node = new_node(NodeKind::Le, add(token_list, i)?, node);
        } else {
            return Ok(node);
        }
    }
}

fn add(token_list: &Vec<Token>, i: &mut usize) -> Result<Node> {
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
    if consume_ident(&token_list[*i]) {
        let offset =
            ((token_list[*i].str.chars().next().unwrap() as u32 - 'a' as u32 + 1) * 8) as i64;
        *i += 1;
        return Ok(new_node_lvar(offset));
    }
    let num = expect_number(&token_list[*i])?;
    *i += 1;
    Ok(new_node_num(num))
}
