use super::compile::*;
use crate::assertions::{EvalExpr, Operator};
use parity_wasm::elements::Instruction;
use EvalExpr::*;

pub fn binary_expr(op: &str, l: i32, r: i32) -> EvalExpr {
    BinaryExpr {
        op: match op {
            "+" => Operator::Plus,
            "-" => Operator::Minus,
            ">" => Operator::GT,
            "<" => Operator::LT,
            _ => panic!(),
        },
        left: expr_int(l),
        right: expr_int(r),
    }
}

fn expr_int(i: i32) -> Box<EvalExpr> {
    Box::new(I32(i))
}

#[test]
fn const_test() {
    let exp = I32(27i32);
    let v = exp.compile(Compiling::default());
    assert_eq!(v, vec![Instruction::I32Const(27)].into())
}

#[test]
fn const_add() {
    let exp = binary_expr("+", 1, 3);
    let v = exp.compile(Compiling::default());
    assert_eq!(
        v,
        vec![
            Instruction::I32Const(1),
            Instruction::I32Const(3),
            Instruction::I32Add
        ]
        .into()
    )
}

#[test]
fn const_add_sub() {
    let exp = BinaryExpr {
        op: Operator::Minus,
        left: Box::new(binary_expr("+", 1, 3)),
        right: Box::new(I32(2)),
    };
    let v = exp.compile(Compiling::default());
    assert_eq!(
        v,
        vec![
            Instruction::I32Const(1),
            Instruction::I32Const(3),
            Instruction::I32Add,
            Instruction::I32Const(2),
            Instruction::I32Sub
        ]
        .into()
    )
}

#[test]
fn const_gt() {
    let exp = binary_expr(">", 1, 3);
    let v = exp.compile(Compiling::default());
    assert_eq!(
        v,
        vec![
            Instruction::I32Const(1),
            Instruction::I32Const(3),
            Instruction::I32GtS
        ]
        .into()
    )
}

#[test]
fn const_gt_lt() {
    let exp = BinaryExpr {
        op: Operator::GT,
        left: Box::new(binary_expr("<", 1, 3)),
        right: Box::new(I32(2)),
    };
    let v = exp.compile(Compiling::default());
    println!("{:?}", v);
    assert_eq!(
        v,
        vec![
            Instruction::I32Const(1),
            Instruction::I32Const(3),
            Instruction::I32LtS,
            Instruction::I32Const(2),
            Instruction::I32GtS
        ]
        .into()
    )
}

mod parser;
mod variable;
