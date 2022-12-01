use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub lang_int);

pub mod ast {
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn smoketest_parser() {
        assert!(lang_int::LangIntParser::new().parse("42;42").is_ok());
    }
}
