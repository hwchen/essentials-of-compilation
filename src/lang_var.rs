//! There seem to be some issues w/ terminating lines w/ `\n`, so I' switched to semicolons.

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub lang_var);

pub mod ast {
    #[derive(Debug)]
    pub enum Expr {
        Number(i64),
        Var(String),
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
        Assign(String, Box<Expr>),
    }

    #[derive(Debug)]
    pub struct LangVar {
        pub stmts: Vec<Stmt>,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn smoketest() {
        assert!(lang_var::LangVarParser::new().parse("y = 10").is_ok());
        assert!(lang_var::LangVarParser::new().parse("print(-y)").is_ok());

        dbg!(lang_var::LangVarParser::new()
            .parse("y = 10;print(-y)")
            .unwrap());
        assert!(lang_var::LangVarParser::new()
            .parse("y = 10;print(-y)")
            .is_ok());
    }
}
