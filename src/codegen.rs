use crate::parse::{Node, NodeKind};

fn gen_lval(node: Node) {
    if node.kind != NodeKind::Lvar {
        panic!(" Left side value is not local value");
    }

    println!("  mov t0, fp");
    println!("  addi t0, t0, -{}", node.offset);
    println!("  addi sp, sp, -4");
    println!("  sw t0, sp, 0",);
}

pub fn gen(node: Node) {
    match node.kind {
        NodeKind::Num => {
            println!("  addi sp, sp, -4");
            println!("  addi t0, r0, {}", node.val);
            println!("  sw t0, sp, 0",);
            return;
        }
        NodeKind::Lvar => {
            gen_lval(node);
            println!("  lw t0, sp, 0");
            println!("  lw t1, t0, 0");
            println!("  sw t1, sp, 0");
            return;
        }
        NodeKind::Assign => {
            gen_lval(*node.lhs.unwrap());
            gen(*node.rhs.unwrap());

            println!("  lw a1, sp, 0");
            println!("  lw a0, sp, 4");
            println!("  sw a1, a0, 0");
            println!("  addi sp, sp, 4");
            return;
        }
        NodeKind::Return => {
            gen(*node.lhs.unwrap());
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
