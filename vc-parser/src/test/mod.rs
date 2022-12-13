use crate::assertions::compile::{Compile, Compiling};
use crate::assertions::parser::program;
use crate::wasm::*;
use metered_wasmi::RuntimeValue;
use parity_wasm::elements::Module;

#[test]
fn string_to_result_add_sub() {
    string_to_result("return 1+3-2", 2)
}

#[test]
fn string_to_result_gt() {
    string_to_result("return 1>3", 2)
}

fn string_to_result(s: &str, re: i32) {
    let exp = program(s);
    assert!(exp.is_ok());
    let r = exp.unwrap();
    println!("{:?}", r);
    let ins = r.1.compile(Compiling::default());
    println!("{:?}", ins.instructions);
    println!("{:?}", ins.locals);
    let module = module_by_compiling(ins);
    assert!(module.sections().len() > 0);
    let result = run_module(module);
    match result {
        Ok(RuntimeValue::I32(rv)) => assert_eq!(rv, re),
        r => panic!("{:?}", r),
    }
}

fn string_to_module(s: &str) -> Module {
    let exp = program(s);
    assert!(exp.is_ok());
    let r = exp.unwrap();
    println!("{:?}", r);
    let ins = r.1.compile(Compiling::default());
    println!("{:?}", ins.instructions);
    println!("{:?}", ins.locals);
    let module = module_by_compiling(ins);
    assert!(module.sections().len() > 0);
    return module;
}

fn single_true_module() -> Module {
    string_to_module("return 4+5-8")
}

fn single_false_module() -> Module {
    string_to_module("return 4+5-9")
}

#[test]
fn test_single_true_flase() {
    let r = eval_single_module(single_true_module());
    println!("r is {}", r);
    assert!(r);

    let r2 = eval_single_module(single_false_module());
    println!("r2 is {}", r2);
    assert!(!r2);
}

#[test]
fn test_and_modules() {
    let m_list = vec![single_true_module(), single_false_module()];
    let t = eval_and_modules(m_list);
    println!("t is: {}", t);
    assert!(!t);
}

#[test]
fn test_or_modules() {
    let m_list = vec![single_true_module(), single_false_module()];
    let t = eval_or_modules(m_list);
    println!("t is: {}", t);
    assert!(t);
}

mod error;
mod variable;
