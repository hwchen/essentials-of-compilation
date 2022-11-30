#[derive(Debug)]
pub enum Expr {
    Number(i64),
    BinaryOp(Box<Expr>, OpCode, Box<Expr>),
    UnaryOp(OpCode, Box<Expr>),
    Group(Box<Expr>),

    InputInt,
}

#[derive(Debug)]
pub enum OpCode {
    Add,
    Sub,
}

#[derive(Debug)]
pub enum Stmt {
    Print(Box<Expr>),
    Expr(Box<Expr>),
}

#[derive(Debug)]
pub struct LangInt {
    pub stmts: Vec<Stmt>,
}
