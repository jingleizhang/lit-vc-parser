#[derive(PartialEq, Debug, Clone)]
pub enum Operator {
    Plus,
    Minus,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Expr {
    Eval(Box<EvalExpr>),
    VarDef(String),
    Assign(String, Box<EvalExpr>),
    Return(Box<EvalExpr>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum EvalExpr {
    I32(i32),
    BinaryExpr {
        op: Operator,
        left: Box<EvalExpr>,
        right: Box<EvalExpr>,
    },
    Variable(String),
}
#[derive(PartialEq, Debug, Clone)]
pub struct Statement(pub Expr);

pub mod compile;
pub mod parser;

#[cfg(test)]
mod test;
