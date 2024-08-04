// operator/control.rs

use crate::environment::Environment;
use crate::exception::LispError;
use crate::expression::Expr;
use crate::evaluator::Evaluator;

pub struct Control;

impl Control {
    pub fn eval_cond(args: &[Expr], env: &mut Environment) -> Result<Expr, LispError> {
        for expr in args {
            match expr {
                Expr::List(list) if list.len() == 2 => {
                    let test = Evaluator::eval_tree(&list[0], env)?;
                    if test != Expr::List(vec![]) && test != Expr::Str("false".to_string()) {
                        return Evaluator::eval_tree(&list[1], env);
                    }
                }
                _ => return Err(LispError::new("cond: invalid clause")),
            }
        }
        Ok(Expr::List(vec![])) // 返回空列表作为缺省值
    }
}
