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

pub mod interp {
    use std::collections::HashMap;
    use std::io::Write;

    use super::ast::*;

    use crate::input_int;

    /// map of variable to value, for looking up assignments
    pub type Env = HashMap<String, i64>;

    pub fn interp<W: Write>(p: LangVar, wtr: &mut W) {
        let mut env = HashMap::new();
        interp_stmts(&p.stmts, &mut env, wtr);
    }

    pub fn interp_stmts<W: Write>(stmts: &[Stmt], env: &mut Env, wtr: &mut W) {
        if stmts.is_empty() {
            return; // return 0?
        } else if stmts.len() == 1 {
            return interp_stmt(&stmts[0], env, &[], wtr);
        } else {
            return interp_stmt(&stmts[1], env, &stmts[1..], wtr);
        }
    }

    fn interp_stmt<W: Write>(s: &Stmt, env: &mut Env, cont: &[Stmt], wtr: &mut W) {
        match s {
            Stmt::Print(e) => writeln!(wtr, "{}", interp_exp(e)).unwrap(),
            Stmt::Expr(e) => {
                interp_exp(e);
            }
            Stmt::Assign(_, _) => todo!("langvar"),
        }

        interp_stmts(cont, env, wtr)
    }

    fn interp_exp(e: &Expr) -> i64 {
        match e {
            Expr::BinaryOp(lhs, op, rhs) => {
                let lhs = interp_exp(lhs);
                let rhs = interp_exp(rhs);

                match op {
                    OpCode::Add => lhs + rhs,
                    OpCode::Sub => lhs - rhs,
                }
            }
            Expr::UnaryOp(op, e) => match op {
                OpCode::Add => interp_exp(e),
                OpCode::Sub => interp_exp(e) * -1,
            },
            Expr::Group(e) => interp_exp(e),
            Expr::Number(n) => *n,
            Expr::InputInt => input_int(),
            Expr::Var(_) => todo!("langvar"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn smoketest_parser() {
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
