use crate::token::{Token, TokenKind};

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

fn consume_return(token: &Token) -> bool {
    if token.kind != TokenKind::Return {
        return false;
    }
    true
}

fn expect(token: &Token, op: &str) {
    if token.kind != TokenKind::Reserved || token.str != op {
        panic!(" It is not {}", op);
    }
}

fn expect_number(token: &Token) -> i64 {
    if token.kind != TokenKind::Num {
        panic!(" It is not number {}", token.str);
    }
    token.val
}

fn at_eof(token: &Token) -> bool {
    token.kind == TokenKind::Eof
}

#[derive(Debug, PartialEq, Clone)]
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
    Return,
    Num,
}
#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct LVar {
    pub name: String,
    pub offset: i64,
}

#[derive(Debug)]
pub struct Parser {
    pub token_list: Vec<Token>,
    pub i: usize,
    pub node_list: Vec<Node>,
    pub lvar_list: Vec<LVar>,
    pub locals: LVar,
}

impl Parser {
    pub fn new(token_list: Vec<Token>) -> Self {
        Self {
            token_list,
            i: 0,
            node_list: Vec::new(),
            lvar_list: Vec::new(),
            locals: LVar {
                name: "".to_string(),
                offset: 0,
            },
        }
    }

    fn find_lvar(&self, token: &Token) -> Option<&LVar> {
        self.lvar_list.iter().find(|&lvar| lvar.name == token.str)
    }

    pub fn program(&mut self) -> Vec<Node> {
        while !at_eof(&self.token_list[self.i]) {
            let node = self.stmt();
            self.node_list.push(node);
        }
        self.node_list.clone()
    }

    fn stmt(&mut self) -> Node {
        let node: Node = if consume_return(&self.token_list[self.i]) {
            self.i += 1;
            Node {
                kind: NodeKind::Return,
                lhs: Some(Box::new(self.expr())),
                rhs: None,
                val: 0,
                offset: 0,
            }
        } else {
            self.expr()
        };
        expect(&self.token_list[self.i], ";");
        self.i += 1;
        node
    }

    fn expr(&mut self) -> Node {
        self.assign()
    }

    fn assign(&mut self) -> Node {
        let mut node = self.equality();
        if consume(&self.token_list[self.i], "=") {
            self.i += 1;
            node = new_node(NodeKind::Assign, node, self.assign());
        }
        node
    }

    fn equality(&mut self) -> Node {
        let mut node = self.relational();
        loop {
            if consume(&self.token_list[self.i], "==") {
                self.i += 1;
                node = new_node(NodeKind::Eq, node, self.relational());
            } else if consume(&self.token_list[self.i], "!=") {
                self.i += 1;
                node = new_node(NodeKind::Ne, node, self.relational());
            } else {
                return node;
            }
        }
    }

    fn relational(&mut self) -> Node {
        let mut node = self.add();
        loop {
            if consume(&self.token_list[self.i], "<") {
                self.i += 1;
                node = new_node(NodeKind::Lt, node, self.add());
            } else if consume(&self.token_list[self.i], "<=") {
                self.i += 1;
                node = new_node(NodeKind::Le, node, self.add());
            } else if consume(&self.token_list[self.i], ">") {
                self.i += 1;
                node = new_node(NodeKind::Lt, self.add(), node);
            } else if consume(&self.token_list[self.i], ">=") {
                self.i += 1;
                node = new_node(NodeKind::Le, self.add(), node);
            } else {
                return node;
            }
        }
    }

    fn add(&mut self) -> Node {
        let mut node = self.mul();
        loop {
            if consume(&self.token_list[self.i], "+") {
                self.i += 1;
                node = new_node(NodeKind::Add, node, self.mul());
            } else if consume(&self.token_list[self.i], "-") {
                self.i += 1;
                node = new_node(NodeKind::Sub, node, self.mul());
            } else {
                return node;
            }
        }
    }

    fn mul(&mut self) -> Node {
        let mut node = self.unary();
        loop {
            if consume(&self.token_list[self.i], "*") {
                self.i += 1;
                node = new_node(NodeKind::Mul, node, self.mul());
            } else if consume(&self.token_list[self.i], "/") {
                self.i += 1;
                node = new_node(NodeKind::Div, node, self.mul());
            } else {
                return node;
            }
        }
    }

    fn unary(&mut self) -> Node {
        if consume(&self.token_list[self.i], "+") {
            self.i += 1;
            return self.primary();
        }
        if consume(&self.token_list[self.i], "-") {
            self.i += 1;
            return new_node(NodeKind::Sub, new_node_num(0), self.primary());
        }
        self.primary()
    }

    fn primary(&mut self) -> Node {
        if consume(&self.token_list[self.i], "(") {
            self.i += 1;
            let node = self.expr();
            expect(&self.token_list[self.i], ")");
            self.i += 1;
            return node;
        }
        if consume_ident(&self.token_list[self.i]) {
            if let Some(lvar) = self.find_lvar(&self.token_list[self.i]) {
                let offset = lvar.offset;
                self.i += 1;
                return new_node_lvar(offset);
            } else {
                self.locals = LVar {
                    name: self.token_list[self.i].str.clone(),
                    offset: self.locals.offset + 4,
                };
                self.lvar_list.push(self.locals.clone());
                let offset = self.locals.offset;
                self.i += 1;
                return new_node_lvar(offset);
            }
        }
        let num = expect_number(&self.token_list[self.i]);
        self.i += 1;
        new_node_num(num)
    }
}
