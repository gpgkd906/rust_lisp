// operator/comparison.rs

use crate::environment::Environment;
use crate::exception::LispError;
use crate::expression::Expr;
use crate::evaluator::Evaluator;

pub struct Comparison;

impl Comparison {
    pub fn eval_greater(args: &[Expr], env: &mut Environment) -> Result<Expr, LispError> {
        if args.len() != 2 {
            return Err(LispError::new("`>` expects exactly two arguments"));
        }

        let left = Evaluator::eval_tree(&args[0], env)?;
        let right = Evaluator::eval_tree(&args[1], env)?;

        match (left, right) {
            (Expr::Number(l), Expr::Number(r)) => {
                if l > r {
                    Ok(Expr::Symbol("t".to_string()))
                } else {
                    Ok(Expr::List(vec![]))
                }
            }
            _ => Err(LispError::new("`>` arguments must be numbers")),
        }
    }

    pub fn eval_greater_equal(args: &[Expr], env: &mut Environment) -> Result<Expr, LispError> {
        if args.len() != 2 {
            return Err(LispError::new("`>=` expects exactly two arguments"));
        }

        let left = Evaluator::eval_tree(&args[0], env)?;
        let right = Evaluator::eval_tree(&args[1], env)?;

        match (left, right) {
            (Expr::Number(l), Expr::Number(r)) => {
                if l >= r {
                    Ok(Expr::Symbol("t".to_string()))
                } else {
                    Ok(Expr::List(vec![]))
                }
            }
            _ => Err(LispError::new("`>=` arguments must be numbers")),
        }
    }

    pub fn eval_less(args: &[Expr], env: &mut Environment) -> Result<Expr, LispError> {
        if args.len() != 2 {
            return Err(LispError::new("`<` expects exactly two arguments"));
        }

        let left = Evaluator::eval_tree(&args[0], env)?;
        let right = Evaluator::eval_tree(&args[1], env)?;

        match (left, right) {
            (Expr::Number(l), Expr::Number(r)) => {
                if l < r {
                    Ok(Expr::Symbol("t".to_string()))
                } else {
                    Ok(Expr::List(vec![]))
                }
            }
            _ => Err(LispError::new("`<` arguments must be numbers")),
        }
    }

    pub fn eval_less_equal(args: &[Expr], env: &mut Environment) -> Result<Expr, LispError> {
        if args.len() != 2 {
            return Err(LispError::new("`<=` expects exactly two arguments"));
        }

        let left = Evaluator::eval_tree(&args[0], env)?;
        let right = Evaluator::eval_tree(&args[1], env)?;

        match (left, right) {
            (Expr::Number(l), Expr::Number(r)) => {
                if l <= r {
                    Ok(Expr::Symbol("t".to_string()))
                } else {
                    Ok(Expr::List(vec![]))
                }
            }
            _ => Err(LispError::new("`<=` arguments must be numbers")),
        }
    }

    pub fn eval_equal(args: &[Expr], env: &mut Environment) -> Result<Expr, LispError> {
        if args.len() != 2 {
            return Err(LispError::new("`==` expects exactly two arguments"));
        }

        let left = Evaluator::eval_tree(&args[0], env)?;
        let right = Evaluator::eval_tree(&args[1], env)?;

        match (left, right) {
            (Expr::Number(l), Expr::Number(r)) => {
                if l == r {
                    Ok(Expr::Symbol("t".to_string()))
                } else {
                    Ok(Expr::List(vec![]))
                }
            }
            (Expr::Symbol(l), Expr::Symbol(r)) => {
                if l == r {
                    Ok(Expr::Symbol("t".to_string()))
                } else {
                    Ok(Expr::List(vec![]))
                }
            }
            (Expr::List(l), Expr::List(r)) => {
                if l == r {
                    Ok(Expr::Symbol("t".to_string()))
                } else {
                    Ok(Expr::List(vec![]))
                }
            }
            _ => Ok(Expr::List(vec![])),
        }
    }

    pub fn eval_not_equal(args: &[Expr], env: &mut Environment) -> Result<Expr, LispError> {
        if args.len() != 2 {
            return Err(LispError::new("`!=` expects exactly two arguments"));
        }

        let left = Evaluator::eval_tree(&args[0], env)?;
        let right = Evaluator::eval_tree(&args[1], env)?;

        match (left, right) {
            (Expr::Number(l), Expr::Number(r)) => {
                if l != r {
                    Ok(Expr::Symbol("t".to_string()))
                } else {
                    Ok(Expr::List(vec![]))
                }
            }
            (Expr::Symbol(l), Expr::Symbol(r)) => {
                if l != r {
                    Ok(Expr::Symbol("t".to_string()))
                } else {
                    Ok(Expr::List(vec![]))
                }
            }
            (Expr::List(l), Expr::List(r)) => {
                if l != r {
                    Ok(Expr::Symbol("t".to_string()))
                } else {
                    Ok(Expr::List(vec![]))
                }
            }
            _ => Ok(Expr::Symbol("t".to_string())),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::environment::Environment;
    use crate::expression::Expr;

    fn setup_environment() -> Environment {
        let mut env = Environment::initialize();
        env.set_symbol("nil".to_string(), Expr::List(vec![])); // 将 nil 设为空列表
        env.set_symbol("t".to_string(), Expr::Symbol("t".to_string())); // 真值 t
        env.set_symbol("T".to_string(), Expr::Symbol("T".to_string())); // 大写 T 作为真值
        env.set_symbol("a".to_string(), Expr::Symbol("a".to_string())); // 定义符号 a
        env.set_symbol("b".to_string(), Expr::Symbol("b".to_string())); // 定义符号 b
        env
    }

    #[test]
    fn test_greater_operator() {
        let mut env = setup_environment();

        let result = Comparison::eval_greater(&[Expr::Number(5), Expr::Number(3)], &mut env);
        assert_eq!(result, Ok(Expr::Symbol("t".to_string())));

        let result = Comparison::eval_greater(&[Expr::Number(2), Expr::Number(3)], &mut env);
        assert_eq!(result, Ok(Expr::List(vec![])));

        let result = Comparison::eval_greater(&[Expr::Number(3), Expr::Number(3)], &mut env);
        assert_eq!(result, Ok(Expr::List(vec![])));
    }

    #[test]
    fn test_greater_equal_operator() {
        let mut env = setup_environment();

        let result = Comparison::eval_greater_equal(&[Expr::Number(5), Expr::Number(3)], &mut env);
        assert_eq!(result, Ok(Expr::Symbol("t".to_string())));

        let result = Comparison::eval_greater_equal(&[Expr::Number(3), Expr::Number(3)], &mut env);
        assert_eq!(result, Ok(Expr::Symbol("t".to_string())));

        let result = Comparison::eval_greater_equal(&[Expr::Number(2), Expr::Number(3)], &mut env);
        assert_eq!(result, Ok(Expr::List(vec![])));
    }

    #[test]
    fn test_less_operator() {
        let mut env = setup_environment();

        let result = Comparison::eval_less(&[Expr::Number(2), Expr::Number(3)], &mut env);
        assert_eq!(result, Ok(Expr::Symbol("t".to_string())));

        let result = Comparison::eval_less(&[Expr::Number(3), Expr::Number(3)], &mut env);
        assert_eq!(result, Ok(Expr::List(vec![])));

        let result = Comparison::eval_less(&[Expr::Number(5), Expr::Number(3)], &mut env);
        assert_eq!(result, Ok(Expr::List(vec![])));
    }

    #[test]
    fn test_less_equal_operator() {
        let mut env = setup_environment();

        let result = Comparison::eval_less_equal(&[Expr::Number(2), Expr::Number(3)], &mut env);
        assert_eq!(result, Ok(Expr::Symbol("t".to_string())));

        let result = Comparison::eval_less_equal(&[Expr::Number(3), Expr::Number(3)], &mut env);
        assert_eq!(result, Ok(Expr::Symbol("t".to_string())));

        let result = Comparison::eval_less_equal(&[Expr::Number(5), Expr::Number(3)], &mut env);
        assert_eq!(result, Ok(Expr::List(vec![])));
    }

    #[test]
    fn test_equal_operator() {
        let mut env = setup_environment();

        let result = Comparison::eval_equal(&[Expr::Number(3), Expr::Number(3)], &mut env);
        assert_eq!(result, Ok(Expr::Symbol("t".to_string())));

        let result = Comparison::eval_equal(&[Expr::Number(3), Expr::Number(5)], &mut env);
        assert_eq!(result, Ok(Expr::List(vec![])));

        let result = Comparison::eval_equal(&[Expr::Symbol("a".to_string()), Expr::Symbol("a".to_string())], &mut env);
        assert_eq!(result, Ok(Expr::Symbol("t".to_string())));

        let result = Comparison::eval_equal(&[Expr::Symbol("a".to_string()), Expr::Symbol("b".to_string())], &mut env);
        assert_eq!(result, Ok(Expr::List(vec![])));
    }

    #[test]
    fn test_not_equal_operator() {
        let mut env = setup_environment();

        let result = Comparison::eval_not_equal(&[Expr::Number(3), Expr::Number(5)], &mut env);
        assert_eq!(result, Ok(Expr::Symbol("t".to_string())));

        let result = Comparison::eval_not_equal(&[Expr::Number(3), Expr::Number(3)], &mut env);
        assert_eq!(result, Ok(Expr::List(vec![])));

        let result = Comparison::eval_not_equal(&[Expr::Symbol("a".to_string()), Expr::Symbol("b".to_string())], &mut env);
        assert_eq!(result, Ok(Expr::Symbol("t".to_string())));

        let result = Comparison::eval_not_equal(&[Expr::Symbol("a".to_string()), Expr::Symbol("a".to_string())], &mut env);
        assert_eq!(result, Ok(Expr::List(vec![])));
    }

    // #[test]
    // fn test_comparison_with_invalid_arguments() {
    //     let mut env = setup_environment();

    //     let result = Comparison::eval_greater(&[Expr::Symbol("a".to_string()), Expr::Number(3)], &mut env);
    //     assert!(result.is_err());

    //     let result = Comparison::eval_greater_equal(&[Expr::Number(3)], &mut env);
    //     assert!(result.is_err());

    //     let result = Comparison::eval_less(&[], &mut env);
    //     assert!(result.is_err());

    //     let result = Comparison::eval_less_equal(&[Expr::Number(3), Expr::Symbol("b".to_string())], &mut env);
    //     assert!(result.is_err());

    //     // Test equality with empty list should be false
    //     let result = Comparison::eval_equal(&[Expr::List(vec![]), Expr::List(vec![Expr::Number(1)])], &mut env);
    //     assert_eq!(result, Ok(Expr::List(vec![])));

    //     // Test not equality with non-empty lists
    //     let result = Comparison::eval_not_equal(&[Expr::List(vec![Expr::Number(1)]), Expr::List(vec![Expr::Number(1)])], &mut env);
    //     assert_eq!(result, Ok(Expr::List(vec![])));
    // }
}
