use crate::assertions::{parser::*, EvalExpr, Expr, Operator};
use core::fmt::Debug;
use EvalExpr::*;
use Expr::*;

#[test]
fn error_block() {
    let s = "1+2
    &%";
    let re = program(s);
    assert!(re.is_err());
}

#[test]
fn error_block_2() {
    let s = "1+2
    --";
    let re = program(s);
    assert!(re.is_err());
}

#[test]
fn number_simple() {
    let r = liberal_parse("1234");
    ok_eq(r, I32(1234));
    let r0 = liberal_parse("02");
    ok_eq(r0, I32(2));
    let re = liberal_parse("K");
    assert!(re.is_err());
}

#[test]
fn symbol_simple() {
    let r = operator("+");
    ok_eq(r, Operator::Plus);
}

fn sample_node() -> EvalExpr {
    (BinaryExpr {
        op: Operator::Minus,
        left: Box::new(binary_expr("+", 1, 3)),
        right: Box::new(I32(2)),
    })
    .clone()
}

#[test]
fn add_sub_simple() {
    let r = compute_parser("1+2");
    ok_eq(r, binary_expr("+", 1, 2));
    let r1 = compute_parser("1 + 3");
    assert!(r1.is_ok());
    let r2 = compute_parser("1 +3 -2");
    assert!(r2.is_ok());
    ok_eq(r2, sample_node());
}

#[test]
fn some_case_for_ws() {
    let strings = vec!["1+3-2", " 1+3-2 ", "1+ 3-2"];
    for text in strings.into_iter() {
        let r2 = statement(text);
        ok_eq(r2, Eval(Box::new(sample_node())));
    }
}

#[test]
fn statement_all() {
    let str = " 1+2 ";
    let ast = statement(str);
    let r = binary_expr("+", 1, 2);
    ok_eq(ast, Eval(Box::new(r)));
    let str = " 1+2 \r\n";
    let ast = statement(str);
    let r = binary_expr("+", 1, 2);
    ok_eq(ast, Eval(Box::new(r)));
}

fn expr_int(i: i32) -> Box<EvalExpr> {
    Box::new(I32(i))
}

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

pub fn ok_eq<T, E>(r: Result<(&str, T), E>, eq: T)
where
    T: PartialEq + Debug,
    E: Debug,
{
    println!("{:?}", r);
    assert!(r.is_ok());
    println!("{:?}", eq);
    assert_eq!(eq, r.ok().unwrap().1);
}

#[test]
fn simple_let() {
    let s = "let a";
    let ast = def_parser(s);
    ok_eq(ast, VarDef("a".to_string()));
}
#[test]
fn simple_variable() {
    let s = " a";
    let ast = variable_parser(s);
    ok_eq(ast, Variable("a".to_string()));
}

#[test]
fn var_in_add_sub() {
    let s = " a+1";
    let ast = compute_parser(s);
    ok_eq(
        ast,
        BinaryExpr {
            op: Operator::Plus,
            left: Box::new(Variable("a".to_string())),
            right: Box::new(I32(1)),
        },
    );
}
#[test]
fn var_in_add_sub_three() {
    let s = "b- a+1";
    let ast = compute_parser(s);
    let ast1 = BinaryExpr {
        op: Operator::Minus,
        left: Box::new(Variable("b".to_string())),
        right: Box::new(Variable("a".to_string())),
    };
    ok_eq(
        ast,
        BinaryExpr {
            op: Operator::Plus,
            left: Box::new(ast1),
            right: Box::new(I32(1)),
        },
    );
}
#[test]
fn assign_simple() {
    let s = "a = b+1";
    let ast = assign_par(s);
    ok_eq(
        ast,
        Assign(
            "a".to_string(),
            Box::new(BinaryExpr {
                op: Operator::Plus,
                left: Box::new(Variable("b".to_string())),
                right: Box::new(I32(1)),
            }),
        ),
    );
}
