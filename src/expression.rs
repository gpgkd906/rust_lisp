// expression.rs

use std::fmt;

#[derive(Clone, Debug)]
pub enum Expr {
    Symbol(String),
    Number(i64),
    Str(String),
    List(Vec<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Symbol(s) => write!(f, "{}", s),
            Expr::Number(n) => write!(f, "{}", n),
            Expr::Str(s) => write!(f, "\"{}\"", s),
            Expr::List(list) => {
                let list_str: Vec<String> = list.iter().map(|expr| format!("{}", expr)).collect();
                write!(f, "({})", list_str.join(" "))
            }
        }
    }
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Expr::Symbol(a), Expr::Symbol(b)) => a == b,
            (Expr::Number(a), Expr::Number(b)) => a == b,
            (Expr::Str(a), Expr::Str(b)) => a == b,
            (Expr::List(a), Expr::List(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for Expr {}
