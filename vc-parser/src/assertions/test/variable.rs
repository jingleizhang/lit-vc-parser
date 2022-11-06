use super::super::{
    compile::{Compile, Compiling},
    EvalExpr, Expr,
};
use crate::{assertions::Operator, Error};
use parity_wasm::elements::Instruction::*;
use EvalExpr::*;
use Expr::*;

#[test]
fn var_def_compile() {
    let ast = VarDef("foo".to_string());
    let result = ast.compile(Compiling::default());
    assert_eq!(
        result,
        Compiling {
            locals: vec!["foo".to_string()],
            ..Compiling::default()
        }
    );
}
#[test]
fn variable_compile() {
    let ast = VarDef("foo".to_string());
    let ast2 = VarDef("bar".to_string());
    let result = ast.compile(Compiling::default());
    let r = ast2.compile(result);
    assert_eq!(
        r,
        Compiling {
            locals: vec!["foo".to_string(), "bar".to_string()],
            ..Compiling::default()
        }
    );
}
#[test]
fn variable_existed_error() {
    let ast = VarDef("foo".to_string());
    let ast2 = VarDef("foo".to_string());
    let result = ast.compile(Compiling::default());
    let r = ast2.compile(result);
    assert_eq!(
        r,
        Compiling {
            locals: vec!["foo".to_string()],
            errors: vec![Error::CompileError("existed var - foo".to_string())],
            ..Compiling::default()
        }
    );
}

#[test]
fn variable_and_get() {
    let ast = VarDef("foo".to_string());
    let ast2 = Variable("foo".to_string());
    let result = ast.compile(Compiling::default());
    let r = ast2.compile(result);
    assert_eq!(
        r,
        Compiling {
            instructions: vec![GetLocal(0)],
            locals: vec!["foo".to_string()],
            ..Compiling::default()
        }
    );
}

#[test]
fn variable_and_unknown() {
    let ast = VarDef("foo".to_string());
    let ast2 = Variable("bar".to_string());
    let result = ast.compile(Compiling::default());
    let r = ast2.compile(result);
    assert_eq!(
        r,
        Compiling {
            locals: vec!["foo".to_string()],
            errors: vec![Error::CompileError("unknown var - bar".to_string())],
            ..Compiling::default()
        }
    );
}

#[test]
fn variable_set() {
    let ast = VarDef("foo".to_string());

    let ast2 = Assign("foo".to_string(), Box::new(I32(42)));
    let result = ast.compile(Compiling::default());
    let r = ast2.compile(result);
    assert_eq!(
        r,
        Compiling {
            instructions: vec![I32Const(42), SetLocal(0)],
            locals: vec!["foo".to_string()],
            ..Compiling::default()
        }
    );
}
#[test]
fn variable_set_unknown() {
    let ast = VarDef("foo".to_string());

    let ast2 = Assign("bar".to_string(), Box::new(I32(42)));
    let result = ast.compile(Compiling::default());
    let r = ast2.compile(result);
    assert_eq!(
        r,
        Compiling {
            instructions: vec![I32Const(42)],
            locals: vec!["foo".to_string()],
            errors: vec![Error::CompileError("unknown var - bar".to_string())],
        }
    );
}
#[test]
fn variable_scene() {
    use Expr::*;
    /*
    let a
    let b
    b=42
    a  = b
    a
     */

    let a1 = VarDef("a".to_string());
    let a2 = VarDef("b".to_string());
    let a3 = Assign("b".to_string(), Box::new(I32(42)));
    let a4 = Assign("a".to_string(), Box::new(Variable("b".to_string())));
    let a5 = Variable("a".to_string());
    let c = vec![a1, a2, a3, a4, Eval(Box::new(a5))]
        .into_iter()
        .fold(Compiling::default(), |c, a| a.compile(c));
    assert_eq!(
        c,
        Compiling {
            instructions: vec![
                I32Const(42),
                SetLocal(1),
                GetLocal(1),
                SetLocal(0),
                GetLocal(0)
            ],
            locals: vec!["a".to_string(), "b".to_string()],
            ..Compiling::default()
        }
    )
}
