use crate::parse::{Node, NodeKind};

pub fn gen(node: Node) {
    if node.kind == NodeKind::Num {
        println!("  addi sp, sp, -4");
        println!("  addi t0, r0, {}", node.val);
        println!("  sw t0, sp, 0",);
        return;
    }

    gen(*node.lhs.unwrap());
    gen(*node.rhs.unwrap());

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
    println!("  sw a0, sp, 0",);
}
