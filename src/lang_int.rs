use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub lang_int);

pub mod ast {
    #[derive(Debug, Clone)]
    pub enum Expr {
        Number(i64),
        BinaryOp(Box<Expr>, OpCode, Box<Expr>),
        UnaryOp(OpCode, Box<Expr>),
        Group(Box<Expr>),

        InputInt,
    }

    #[derive(Debug, Copy, Clone)]
    pub enum OpCode {
        Add,
        Sub,
    }

    #[derive(Debug, Clone)]
    pub enum Stmt {
        Print(Box<Expr>),
        Expr(Box<Expr>),
    }

    #[derive(Debug, Clone)]
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

/// Partial Evaluator
///
/// Need to have the various pe_unary, pe_binary; if just doing match statements in pe_exp, won't
/// recurse in fine-grained enough fashion (need to think about this more)
pub mod pe {
    use super::ast::*;

    pub fn pe(program: &LangInt) -> LangInt {
        LangInt {
            stmts: program.stmts.iter().map(pe_stmt).collect(),
        }
    }

    fn pe_stmt(s: &Stmt) -> Stmt {
        match s {
            Stmt::Print(e) => Stmt::Print(Box::new(pe_exp(e))),
            Stmt::Expr(e) => Stmt::Expr(Box::new(pe_exp(e))),
        }
    }

    fn pe_exp(e: &Expr) -> Expr {
        match e {
            Expr::BinaryOp(ref lhs, op, ref rhs) => pe_binary(&pe_exp(lhs), *op, &pe_exp(rhs)),
            Expr::UnaryOp(op, e) => pe_unary(*op, &pe_exp(e)),
            Expr::Group(e) => pe_exp(&*e),
            Expr::Number(_) => e.clone(),
            Expr::InputInt => e.clone(),
        }
    }

    fn pe_unary(op: OpCode, r: &Expr) -> Expr {
        match r {
            Expr::Number(n) => match op {
                OpCode::Sub => Expr::Number(n * -1),
                _ => r.clone(),
            },
            _ => r.clone(),
        }
    }

    fn pe_binary(r1: &Expr, op: OpCode, r2: &Expr) -> Expr {
        match (r1, r2) {
            (&Expr::Number(n), &Expr::Number(m)) => match op {
                OpCode::Add => Expr::Number(n + m),
                OpCode::Sub => Expr::Number(n - m),
            },
            _ => Expr::BinaryOp(Box::new(r1.clone()), op, Box::new(r2.clone())),
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

    #[test]
    fn smoketest_pe() {
        let input = "print((5 + 10 + (-5)) + (52 - (10 + 10)))";
        {
            let tree = lang_int::LangIntParser::new().parse(input).unwrap();
            let mut output_buf = std::io::Cursor::new(Vec::new());
            interp::interp(tree, &mut output_buf);
            assert_eq!(output_buf.into_inner(), b"42\n");
        }

        {
            let tree = lang_int::LangIntParser::new().parse(input).unwrap();
            let tree = pe::pe(&tree);
            dbg!(&tree);
            let mut output_buf = std::io::Cursor::new(Vec::new());
            interp::interp(tree, &mut output_buf);
            assert_eq!(output_buf.into_inner(), b"42\n");
        }
    }
}
