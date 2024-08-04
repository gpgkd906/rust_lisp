// environment.rs

use std::collections::HashMap;
use crate::expression::Expr;

pub struct Environment {
    symbols: HashMap<String, Expr>,
}

impl Environment {
    pub fn initialize() -> Self {
        let mut env = Environment {
            symbols: HashMap::new(),
        };
        // 预定义一些 Lisp 常用符号
        env.set_symbol("T".to_string(), Expr::Symbol("T".to_string()));
        env.set_symbol("NIL".to_string(), Expr::List(vec![])); // NIL 表示空列表
        env
    }

    pub fn get_symbol(&self, symbol: &str) -> Option<&Expr> {
        self.symbols.get(symbol)
    }

    pub fn set_symbol(&mut self, symbol: String, value: Expr) {
        self.symbols.insert(symbol, value);
    }
}
