use super::{EvalExpr, Expr, Operator, Statement};
use super::{EvalExpr::*, Expr::*};
use crate::Error;
use parity_wasm::elements::Instruction;
use std::iter::empty;

pub trait Compile {
    fn compile(&self, compiling: Compiling) -> Compiling;
}

impl Compile for Statement {
    fn compile(&self, compiling: Compiling) -> Compiling {
        let Statement(e) = self;
        match e {
            Eval(_) => e.compile(compiling).merge(vec![Instruction::Drop].into()),
            _ => e.compile(compiling),
        }
    }
}
impl Compile for Vec<Statement> {
    fn compile(&self, compiling: Compiling) -> Compiling {
        self.into_iter().fold(compiling, |c, a| a.compile(c))
    }
}

impl Compile for EvalExpr {
    fn compile(&self, compiling: Compiling) -> Compiling {
        match self {
            I32(i) => compiling.merge(vec![Instruction::I32Const(*i)].into()),
            BinaryExpr { op, left, right } => {
                let left_compiled = left.compile(compiling.clone());
                let right_compiled = right.compile(left_compiled.clone());
                right_compiled.merge(
                    vec![match op {
                        &Operator::Plus => Instruction::I32Add,
                        &Operator::Minus => Instruction::I32Sub,
                        &Operator::GT => Instruction::I32GtS,
                        &Operator::LT => Instruction::I32LtS,
                    }]
                    .into(),
                )
            }
            Variable(name) => {
                let new = match compiling.local_index(name) {
                    Some(index) => Compiling {
                        instructions: vec![Instruction::GetLocal(index)],
                        ..Compiling::default()
                    },
                    None => Compiling {
                        errors: vec![Error::CompileError(format!("unknown var - {}", name))],
                        ..Compiling::default()
                    },
                };
                compiling.merge(new)
            }
        }
    }
}

impl Compile for Expr {
    fn compile(&self, compiling: Compiling) -> Compiling {
        match self {
            Eval(eval) => eval.compile(compiling),
            Return(ret) => ret.compile(compiling),
            VarDef(name) => {
                let new = match compiling.local_index(name) {
                    None => Compiling {
                        locals: vec![name.clone()],
                        ..Compiling::default()
                    },
                    Some(_) => Compiling {
                        errors: vec![Error::CompileError(format!("existed var - {}", name))],
                        ..Compiling::default()
                    },
                };
                compiling.merge(new)
            }

            Assign(name, value) => {
                let value_compiled = value.compile(compiling.clone());
                let new = match compiling.local_index(name) {
                    Some(index) => Compiling {
                        instructions: vec![Instruction::SetLocal(index)],
                        ..Compiling::default()
                    },
                    None => Compiling {
                        errors: vec![Error::CompileError(format!("unknown var - {}", name))],
                        ..Compiling::default()
                    },
                };
                value_compiled.merge(new)
            } //_ => unimplemented!(),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Compiling {
    pub instructions: Vec<Instruction>,
    pub locals: Vec<String>,
    pub errors: Vec<Error>,
}

impl Compiling {
    pub fn merge(self, other: Compiling) -> Compiling {
        Compiling {
            instructions: self
                .instructions
                .into_iter()
                .chain(other.instructions)
                .collect(),
            locals: {
                let mut re = self
                    .locals
                    .into_iter()
                    .chain(other.locals)
                    .collect::<Vec<String>>();
                re.dedup();
                re
            },
            errors: self.errors.into_iter().chain(other.errors).collect(),
        }
    }
    fn local_index(&self, name: &str) -> Option<u32> {
        self.locals
            .iter()
            .position(|l| l == name)
            .map(|i| i.try_into().unwrap())
    }
}

impl Into<Compiling> for Vec<Instruction> {
    fn into(self) -> Compiling {
        Compiling {
            instructions: empty().chain(self).collect(),
            ..Compiling::default()
        }
    }
}
