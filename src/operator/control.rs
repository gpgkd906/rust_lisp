// operator/control.rs

use crate::environment::Environment;
use crate::exception::LispError;
use crate::expression::Expr;
use crate::evaluator::Evaluator;

pub struct Control;

impl Control {
    pub fn eval_cond(conditions: &[Expr], env: &mut Environment) -> Result<Expr, LispError> {
        for condition in conditions {
            if let Expr::List(pair) = condition {
                if pair.len() != 2 {
                    return Err(LispError::new("Each cond clause must have exactly two elements"));
                }
                let test = &pair[0];
                let result = &pair[1];

                let test_value = match Evaluator::eval_tree(test, env) {
                    Ok(Expr::Symbol(s)) if s == "t" => true,  // 支持真值符号 t
                    Ok(Expr::Symbol(s)) if s == "T" => true, // 大写 T 也作为真值
                    Ok(Expr::Number(n)) if n != 0 => true,   // 非零数值作为真值
                    Ok(Expr::List(list)) if !list.is_empty() => true, // 非空列表为真
                    _ => false,
                };

                if test_value {
                    return Evaluator::eval_tree(result, env);
                }

            } else {
                return Err(LispError::new("Cond clause must be a list"));
            }
        }
        Err(LispError::new("No true condition in cond"))
    }
}
