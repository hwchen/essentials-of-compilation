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

pub mod interp {
    use super::ast::*;

    use crate::input_int;
    use std::io::Write;

    pub fn interp<W: Write>(p: LangInt, wtr: &mut W) {
        for s in p.stmts {
            interp_stmt(s, wtr);
        }
    }

    fn interp_stmt<W: Write>(s: Stmt, wtr: &mut W) {
        match s {
            Stmt::Print(e) => writeln!(wtr, "{}", interp_exp(*e)).unwrap(),
            Stmt::Expr(e) => {
                interp_exp(*e);
            }
        }
    }

    fn interp_exp(e: Expr) -> i64 {
        match e {
            Expr::BinaryOp(lhs, op, rhs) => {
                let lhs = interp_exp(*lhs);
                let rhs = interp_exp(*rhs);

                match op {
                    OpCode::Add => lhs + rhs,
                    OpCode::Sub => lhs - rhs,
                }
            }
            Expr::UnaryOp(op, e) => match op {
                OpCode::Add => interp_exp(*e),
                OpCode::Sub => interp_exp(*e) * -1,
            },
            Expr::Group(e) => interp_exp(*e),
            Expr::Number(n) => n,
            Expr::InputInt => input_int(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn smoketest_parser() {
        assert!(lang_int::LangIntParser::new().parse("42;42").is_ok());
    }

    #[test]
    fn smoketest_interpreter() {
        let tree = lang_int::LangIntParser::new()
            .parse("42;print(10+32)")
            .unwrap();
        let mut output_buf = std::io::Cursor::new(Vec::new());
        interp::interp(tree, &mut output_buf);
        assert_eq!(output_buf.into_inner(), b"42\n");
    }
}
