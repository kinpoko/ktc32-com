use crate::parse::{Node, NodeKind};

#[derive(Debug)]
pub struct CodeGenerator {
    pub count: i64,
}

impl CodeGenerator {
    pub fn new() -> Self {
        Self { count: 0 }
    }

    fn gen_lval(&self, node: &Node) {
        if node.kind != NodeKind::Lvar {
            panic!(" Left side value is not local value");
        }

        println!("  mov t0, fp");
        println!("  addi t0, t0, -{}", node.offset);
        println!("  addi sp, sp, -4");
        println!("  sw t0, sp, 0",);
    }

    pub fn gen(&mut self, node: &Node) {
        match node.kind {
            NodeKind::Num => {
                println!("  addi sp, sp, -4");
                println!("  addi t0, r0, {}", node.val);
                println!("  sw t0, sp, 0",);
                return;
            }
            NodeKind::Lvar => {
                self.gen_lval(node);
                println!("  lw t0, sp, 0");
                println!("  lw t1, t0, 0");
                println!("  sw t1, sp, 0");
                return;
            }
            NodeKind::Assign => {
                self.gen_lval(node.lhs.as_ref().unwrap());
                self.gen(node.rhs.as_ref().unwrap());

                println!("  lw a1, sp, 0");
                println!("  lw a0, sp, 4");
                println!("  sw a1, a0, 0");
                println!("  addi sp, sp, 4");
                return;
            }
            NodeKind::If => {
                self.gen(node.cond.as_ref().unwrap());
                println!("  lw a0, sp, 0");
                println!("  addi sp, sp, 4");
                println!("  addi t0, zero, 1");
                println!("  beq a0, t0, 4");
                println!("  jal zero, else{}", self.count);
                self.gen(node.then.as_ref().unwrap());
                println!("  jal zero, end{}", self.count);
                println!("else{}:", self.count);
                match &node.els {
                    Some(_) => {
                        self.gen(node.els.as_ref().unwrap());
                    }
                    None => {
                        println!("  jal zero, end{}", self.count);
                    }
                }

                println!("end{}:", self.count);
                self.count += 1;
                return;
            }
            NodeKind::Return => {
                self.gen(node.lhs.as_ref().unwrap());
                println!("  lw a0, sp, 0");
                println!("  addi sp, sp, 4");
                println!("  mov sp, fp");
                println!("  lw fp, sp, 0");
                println!("  lw ra, sp, 4");
                println!("  addi sp, sp, 8");
                println!("  jalr zero, ra, 0");
                return;
            }

            _ => {}
        }

        self.gen(node.lhs.as_ref().unwrap());
        self.gen(node.rhs.as_ref().unwrap());

        println!("  addi sp, sp, 8");
        println!("  lw a1, sp, -8");
        println!("  lw a0, sp, -4");

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
            NodeKind::Eq => {
                println!("  mov t0, zero");
                println!("  beq a0, a1, 4");
                println!("  addi t0, t0, -1");
                println!("  addi t0, t0, 1");
                println!("  mov a0, t0");
            }
            NodeKind::Ne => {
                println!("  mov t0, zero");
                println!("  bnq a0, a1, 4");
                println!("  addi t0, t0, -1");
                println!("  addi t0, t0, 1");
                println!("  mov a0, t0");
            }
            NodeKind::Lt => {
                println!("  slt a0, a1");
                println!("  mov a0, flag");
            }
            NodeKind::Le => {
                println!("  mov t0, zero");
                println!("  slt a1, a0");
                println!("  bnq flag, zero, 4");
                println!("  addi t0, zero, 1");
                println!("  mov a0, t0");
            }

            _ => {}
        }
        println!("  addi sp, sp, -4");
        println!("  sw a0, sp, 0");
    }
}
