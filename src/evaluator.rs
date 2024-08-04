// evaluator.rs

use crate::environment::Environment;
use crate::exception::LispError;
use crate::expression::Expr;
use crate::operator::{
    arithmetic::Arithmetic, comparison::Comparison, control::Control, list::ListOps, set::SetOps,
};

pub struct Evaluator;

impl Evaluator {
    pub fn eval_tree(ast: &Expr, env: &mut Environment) -> Result<Expr, LispError> {
        match ast {
            Expr::Symbol(symbol) => {
                env.get_symbol(symbol)
                    .cloned()
                    .ok_or_else(|| LispError::new(&format!("Undefined symbol: {}", symbol)))
            }
            Expr::Number(_) | Expr::Str(_) => Ok(ast.clone()), // Return the same expression for numbers and strings
            Expr::List(list) => {
                if list.is_empty() {
                    return Err(LispError::new("Cannot evaluate an empty list"));
                }
                let first = &list[0];
                match first {
                    Expr::Symbol(s) => match s.as_str() {
                        "+" => Arithmetic::eval_add(&list[1..], env),
                        "-" => Arithmetic::eval_subtract(&list[1..], env),
                        "*" => Arithmetic::eval_multiply(&list[1..], env),
                        "/" => Arithmetic::eval_divide(&list[1..], env),
                        "setf" => SetOps::eval_setf(&list[1..], env),
                        "car" => ListOps::eval_car(&list[1..], env),
                        "cdr" => ListOps::eval_cdr(&list[1..], env),
                        "cons" => ListOps::eval_cons(&list[1..], env),
                        "cond" => Control::eval_cond(&list[1..], env),
                        ">" => Comparison::eval_greater(&list[1..], env),
                        "<" => Comparison::eval_less(&list[1..], env),
                        ">=" => Comparison::eval_greater_equal(&list[1..], env),
                        "<=" => Comparison::eval_less_equal(&list[1..], env),
                        "=" => Comparison::eval_equal(&list[1..], env),
                        "quote" => {
                            if list.len() != 2 {
                                Err(LispError::new("quote requires exactly one argument"))
                            } else {
                                Ok(list[1].clone())
                            }
                        }
                        _ => Err(LispError::new(&format!("Unknown function: {}", s))),
                    },
                    _ => Err(LispError::new("First element in list must be a symbol")),
                }
            }
        }
    }
}
