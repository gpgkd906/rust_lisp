// operator/set.rs

use crate::environment::Environment;
use crate::exception::LispError;
use crate::expression::Expr;
use crate::evaluator::Evaluator;

pub struct SetOps;

impl SetOps {
    pub fn eval_setf(args: &[Expr], env: &mut Environment) -> Result<Expr, LispError> {
        if args.len() != 2 {
            return Err(LispError::new("setf requires exactly two arguments"));
        }

        let symbol = match &args[0] {
            Expr::Symbol(s) => s.clone(),
            _ => return Err(LispError::new("setf: first argument must be a symbol")),
        };

        let value = Evaluator::eval_tree(&args[1], env)?;
        env.set_symbol(symbol.clone(), value.clone());
        Ok(value)
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::environment::Environment;
    use crate::expression::Expr;

    #[test]
    fn test_eval_setf_success() {
        let mut env = Environment::initialize();
        let symbol = "x";
        let value = Expr::Number(42);

        let args = vec![Expr::Symbol(symbol.to_string()), value.clone()];
        let result = SetOps::eval_setf(&args, &mut env);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), value);
        assert_eq!(env.get_symbol(symbol), Some(&value));
    }

    #[test]
    fn test_eval_setf_incorrect_number_of_arguments() {
        let mut env = Environment::initialize();

        // 测试少于两个参数
        let args = vec![Expr::Symbol("x".to_string())];
        let result = SetOps::eval_setf(&args, &mut env);
        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err.to_string(), "setf requires exactly two arguments");
        }

        // 测试多于两个参数
        let args = vec![
            Expr::Symbol("x".to_string()),
            Expr::Number(42),
            Expr::Number(43),
        ];
        let result = SetOps::eval_setf(&args, &mut env);
        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err.to_string(), "setf requires exactly two arguments");
        }
    }

    #[test]
    fn test_eval_setf_first_argument_not_symbol() {
        let mut env = Environment::initialize();

        let args = vec![Expr::Number(42), Expr::Number(43)];
        let result = SetOps::eval_setf(&args, &mut env);

        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err.to_string(), "setf: first argument must be a symbol");
        }
    }

    #[test]
    fn test_eval_setf_eval_value_expression() {
        let mut env = Environment::initialize();
        env.set_symbol("y".to_string(), Expr::Number(10));

        // 设置 x 的值为 y + 32
        let args = vec![
            Expr::Symbol("x".to_string()),
            Expr::List(vec![
                Expr::Symbol("+".to_string()),
                Expr::Symbol("y".to_string()),
                Expr::Number(32),
            ]),
        ];

        // 假设 Evaluator::eval_tree 正确处理了 y + 32 的计算
        let result = SetOps::eval_setf(&args, &mut env);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Expr::Number(42));
        assert_eq!(env.get_symbol("x"), Some(&Expr::Number(42)));
    }
}
