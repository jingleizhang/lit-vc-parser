use assertions::compile::{Compile, Compiling};
use assertions::parser::program;
use metered_wasmi::RuntimeValue;
use nom::Finish;
use serde::Serialize;
use vc::VC;
use wasm::{module_by_compiling, run_module};

#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum Error {
    ParseError(String),
    CompileError(String),
    RuntimeError(String),
}

#[derive(Serialize)]
pub struct CompilingRepresent {
    pub instructions: Vec<String>,
    pub locals: Vec<String>,
    pub errors: Vec<Error>,
}
impl CompilingRepresent {
    pub fn from_compiling(compiling: &Compiling) -> CompilingRepresent {
        let c = compiling.clone();
        CompilingRepresent {
            instructions: c.instructions.iter().map(|s| s.to_string()).collect(),
            locals: c.locals,
            errors: c.errors,
        }
    }
}

pub fn compile_vc(s: &str) -> Result<Compiling, Error> {
    let vc: VC = serde_json::from_str::<VC>(s).unwrap();
    let (_input, exp) = program(&vc.credential_subject.assertions).finish()?;
    Ok(exp.compile(Compiling::default()))
}

pub fn run_vc(s: &str, compiled: Option<Compiling>) -> Result<i32, Error> {
    let compiling = compiled.unwrap_or(compile_vc(s)?);
    if compiling.errors.is_empty() {
        let module = module_by_compiling(compiling.clone());
        println!("{:?}", module);
        let re = run_module(module);
        match re {
            Ok(RuntimeValue::I32(i)) => Ok(i),
            _ => Err(Error::RuntimeError(format!(
                "not supported type - {:?}",
                re
            ))),
        }
    } else {
        Err(Error::CompileError(format!(
            "compiling error - {:?}",
            compiling.errors
        )))
    }
}

impl From<nom::error::Error<&str>> for Error {
    fn from(e: nom::error::Error<&str>) -> Self {
        Error::ParseError(format!("{:?}", e))
    }
}

pub mod assertions;
pub mod vc;
pub mod wasm;

#[cfg(test)]
mod test;
