// operator/comparison.rs
use crate::operator::OperatorRegistry;
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

        let left = Evaluator::eval(&args[0], env)?;
        let right = Evaluator::eval(&args[1], env)?;

        match (left, right) {
            (Expr::Number(l), Expr::Number(r)) => {
                if l > r {
                    Ok(Expr::Symbol("t".to_string()))
                } else {
                    Ok(Expr::List(vec![]))
                }
            }
            (Expr::Float(l), Expr::Float(r)) => {
                if l > r {
                    Ok(Expr::Symbol("t".to_string()))
                } else {
                    Ok(Expr::List(vec![]))
                }
            }
            (Expr::Number(l), Expr::Float(r)) => {
                if (l as f64) > r {
                    Ok(Expr::Symbol("t".to_string()))
                } else {
                    Ok(Expr::List(vec![]))
                }
            }
            (Expr::Float(l), Expr::Number(r)) => {
                if l > (r as f64) {
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

        let left = Evaluator::eval(&args[0], env)?;
        let right = Evaluator::eval(&args[1], env)?;

        match (left, right) {
            (Expr::Number(l), Expr::Number(r)) => {
                if l >= r {
                    Ok(Expr::Symbol("t".to_string()))
                } else {
                    Ok(Expr::List(vec![]))
                }
            }
            (Expr::Float(l), Expr::Float(r)) => {
                if l >= r {
                    Ok(Expr::Symbol("t".to_string()))
                } else {
                    Ok(Expr::List(vec![]))
                }
            }
            (Expr::Number(l), Expr::Float(r)) => {
                if (l as f64) >= r {
                    Ok(Expr::Symbol("t".to_string()))
                } else {
                    Ok(Expr::List(vec![]))
                }
            }
            (Expr::Float(l), Expr::Number(r)) => {
                if l >= (r as f64) {
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

        let left = Evaluator::eval(&args[0], env)?;
        let right = Evaluator::eval(&args[1], env)?;

        match (left, right) {
            (Expr::Number(l), Expr::Number(r)) => {
                if l < r {
                    Ok(Expr::Symbol("t".to_string()))
                } else {
                    Ok(Expr::List(vec![]))
                }
            }
            (Expr::Float(l), Expr::Float(r)) => {
                if l < r {
                    Ok(Expr::Symbol("t".to_string()))
                } else {
                    Ok(Expr::List(vec![]))
                }
            }
            (Expr::Number(l), Expr::Float(r)) => {
                if (l as f64) < r {
                    Ok(Expr::Symbol("t".to_string()))
                } else {
                    Ok(Expr::List(vec![]))
                }
            }
            (Expr::Float(l), Expr::Number(r)) => {
                if l < (r as f64) {
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

        let left = Evaluator::eval(&args[0], env)?;
        let right = Evaluator::eval(&args[1], env)?;

        match (left, right) {
            (Expr::Number(l), Expr::Number(r)) => {
                if l <= r {
                    Ok(Expr::Symbol("t".to_string()))
                } else {
                    Ok(Expr::List(vec![]))
                }
            }
            (Expr::Float(l), Expr::Float(r)) => {
                if l <= r {
                    Ok(Expr::Symbol("t".to_string()))
                } else {
                    Ok(Expr::List(vec![]))
                }
            }
            (Expr::Number(l), Expr::Float(r)) => {
                if (l as f64) <= r {
                    Ok(Expr::Symbol("t".to_string()))
                } else {
                    Ok(Expr::List(vec![]))
                }
            }
            (Expr::Float(l), Expr::Number(r)) => {
                if l <= (r as f64) {
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
            return Err(LispError::new("`eq` expects exactly two arguments"));
        }
    
        let left = Evaluator::eval(&args[0], env)?;
        let right = Evaluator::eval(&args[1], env)?;
    
        match (&left, &right) {
            (Expr::Number(l), Expr::Number(r)) => {
                if l == r {
                    Ok(Expr::Symbol("t".to_string()))
                } else {
                    Ok(Expr::List(vec![]))
                }
            }
            (Expr::Float(l), Expr::Float(r)) => {
                if (l - r).abs() < f64::EPSILON {
                    Ok(Expr::Symbol("t".to_string()))
                } else {
                    Ok(Expr::List(vec![]))
                }
            }
            (Expr::Number(l), Expr::Float(r)) => {
                if ((*l as f64) - r).abs() < f64::EPSILON {
                    Ok(Expr::Symbol("t".to_string()))
                } else {
                    Ok(Expr::List(vec![]))
                }
            }
            (Expr::Float(l), Expr::Number(r)) => {
                if (l - (*r as f64)).abs() < f64::EPSILON {
                    Ok(Expr::Symbol("t".to_string()))
                } else {
                    Ok(Expr::List(vec![]))
                }
            }
            (Expr::Symbol(_), Expr::Symbol(_)) => {
                let left = Evaluator::eval(&left, env);
                let right = Evaluator::eval(&right, env);
                if left == right {
                    Ok(Expr::Symbol("t".to_string()))
                } else {
                    Ok(Expr::List(vec![]))
                }
            }
            (Expr::List(l), Expr::List(r)) => {
                // Check if the lists are the same reference
                if std::ptr::eq(l, r) {
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
            return Err(LispError::new("`ne` expects exactly two arguments"));
        }
    
        let left = Evaluator::eval(&args[0], env)?;
        let right = Evaluator::eval(&args[1], env)?;
    
        match (&left, &right) {
            (Expr::Number(l), Expr::Number(r)) => {
                if l != r {
                    Ok(Expr::Symbol("t".to_string()))
                } else {
                    Ok(Expr::List(vec![]))
                }
            }
            (Expr::Float(l), Expr::Float(r)) => {
                if (l - r).abs() >= f64::EPSILON {
                    Ok(Expr::Symbol("t".to_string()))
                } else {
                    Ok(Expr::List(vec![]))
                }
            }
            (Expr::Number(l), Expr::Float(r)) => {
                if ((*l as f64) - r).abs() >= f64::EPSILON {
                    Ok(Expr::Symbol("t".to_string()))
                } else {
                    Ok(Expr::List(vec![]))
                }
            }
            (Expr::Float(l), Expr::Number(r)) => {
                if (l - (*r as f64)).abs() >= f64::EPSILON {
                    Ok(Expr::Symbol("t".to_string()))
                } else {
                    Ok(Expr::List(vec![]))
                }
            }
            (Expr::Symbol(_), Expr::Symbol(_)) => {
                let left = Evaluator::eval(&left, env);
                let right = Evaluator::eval(&right, env);
                if left != right {
                    Ok(Expr::Symbol("t".to_string()))
                } else {
                    Ok(Expr::List(vec![]))
                }
            }
            (Expr::List(l), Expr::List(r)) => {
                // Check if the lists are not the same reference
                if !std::ptr::eq(l, r) {
                    Ok(Expr::Symbol("t".to_string()))
                } else {
                    Ok(Expr::List(vec![]))
                }
            }
            _ => Ok(Expr::Symbol("t".to_string())),
        }
    }
                    
}

pub fn register_comparison_operators() {
    OperatorRegistry::register(">", Comparison::eval_greater);
    OperatorRegistry::register("gt", Comparison::eval_greater);
    OperatorRegistry::register(">=", Comparison::eval_greater_equal);
    OperatorRegistry::register("gte", Comparison::eval_greater_equal);
    OperatorRegistry::register("<", Comparison::eval_less);
    OperatorRegistry::register("lt", Comparison::eval_less);
    OperatorRegistry::register("<=", Comparison::eval_less_equal);
    OperatorRegistry::register("lte", Comparison::eval_less_equal);
    OperatorRegistry::register("eq", Comparison::eval_equal);
    OperatorRegistry::register("ne", Comparison::eval_not_equal);
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

        // 正常的数字比较
        let result = Comparison::eval_greater(&[Expr::Number(5), Expr::Number(3)], &mut env);
        assert_eq!(result, Ok(Expr::Symbol("t".to_string())));

        // 不大于
        let result = Comparison::eval_greater(&[Expr::Number(2), Expr::Number(3)], &mut env);
        assert_eq!(result, Ok(Expr::List(vec![])));

        // 相等的情况
        let result = Comparison::eval_greater(&[Expr::Number(3), Expr::Number(3)], &mut env);
        assert_eq!(result, Ok(Expr::List(vec![])));

        // 数字与浮点数
        let result = Comparison::eval_greater(&[Expr::Number(4), Expr::Float(3.5)], &mut env);
        assert_eq!(result, Ok(Expr::Symbol("t".to_string())));

        let result = Comparison::eval_greater(&[Expr::Float(4.5), Expr::Number(5)], &mut env);
        assert_eq!(result, Ok(Expr::List(vec![])));

        // 浮点数与小整数
        let result = Comparison::eval_greater(&[Expr::Float(0.1), Expr::Number(0)], &mut env);
        assert_eq!(result, Ok(Expr::Symbol("t".to_string())));
    }

    #[test]
    fn test_greater_operator_with_floats() {
        let mut env = setup_environment();

        // 浮点数比较
        let result = Comparison::eval_greater(&[Expr::Float(5.0), Expr::Number(3)], &mut env);
        assert_eq!(result, Ok(Expr::Symbol("t".to_string())));

        // 不大于浮点数
        let result = Comparison::eval_greater(&[Expr::Float(2.5), Expr::Float(2.6)], &mut env);
        assert_eq!(result, Ok(Expr::List(vec![])));

        // 相等的浮点数
        let result = Comparison::eval_greater(&[Expr::Float(3.0), Expr::Float(3.0)], &mut env);
        assert_eq!(result, Ok(Expr::List(vec![])));

        // 比较小的浮点数
        let result = Comparison::eval_greater(&[Expr::Float(0.00001), Expr::Float(0.000001)], &mut env);
        assert_eq!(result, Ok(Expr::Symbol("t".to_string())));
    }

    #[test]
    fn test_greater_operator_invalid_arguments() {
        let mut env = setup_environment();

        // 非数字类型
        let result = Comparison::eval_greater(&[Expr::Symbol("a".to_string()), Expr::Number(3)], &mut env);
        assert_eq!(result, Err(LispError::new("`>` arguments must be numbers")));

        // 不足的参数数量
        let result = Comparison::eval_greater(&[Expr::Number(5)], &mut env);
        assert_eq!(result, Err(LispError::new("`>` expects exactly two arguments")));
    }

    #[test]
    fn test_greater_equal_operator() {
        let mut env = setup_environment();

        // 大于等于的测试
        let result = Comparison::eval_greater_equal(&[Expr::Number(5), Expr::Number(3)], &mut env);
        assert_eq!(result, Ok(Expr::Symbol("t".to_string())));

        // 相等
        let result = Comparison::eval_greater_equal(&[Expr::Number(3), Expr::Number(3)], &mut env);
        assert_eq!(result, Ok(Expr::Symbol("t".to_string())));

        // 小于
        let result = Comparison::eval_greater_equal(&[Expr::Number(2), Expr::Number(3)], &mut env);
        assert_eq!(result, Ok(Expr::List(vec![])));

        // 数字与浮点数
        let result = Comparison::eval_greater_equal(&[Expr::Number(3), Expr::Float(3.0)], &mut env);
        assert_eq!(result, Ok(Expr::Symbol("t".to_string())));
    }

    #[test]
    fn test_greater_equal_operator_with_floats() {
        let mut env = setup_environment();

        // 浮点数大于等于测试
        let result = Comparison::eval_greater_equal(&[Expr::Float(5.0), Expr::Number(3)], &mut env);
        assert_eq!(result, Ok(Expr::Symbol("t".to_string())));

        // 相等浮点数
        let result = Comparison::eval_greater_equal(&[Expr::Float(2.6), Expr::Float(2.6)], &mut env);
        assert_eq!(result, Ok(Expr::Symbol("t".to_string())));

        // 小于浮点数
        let result = Comparison::eval_greater_equal(&[Expr::Float(2.5), Expr::Float(2.6)], &mut env);
        assert_eq!(result, Ok(Expr::List(vec![])));

        // 浮点数与小整数
        let result = Comparison::eval_greater_equal(&[Expr::Float(0.0), Expr::Number(-1)], &mut env);
        assert_eq!(result, Ok(Expr::Symbol("t".to_string())));
    }

    #[test]
    fn test_greater_equal_operator_invalid_arguments() {
        let mut env = setup_environment();

        // 非数字类型
        let result = Comparison::eval_greater_equal(&[Expr::Symbol("a".to_string()), Expr::Number(3)], &mut env);
        assert_eq!(result, Err(LispError::new("`>=` arguments must be numbers")));

        // 不足的参数数量
        let result = Comparison::eval_greater_equal(&[Expr::Number(5)], &mut env);
        assert_eq!(result, Err(LispError::new("`>=` expects exactly two arguments")));
    }

    #[test]
    fn test_less_operator() {
        let mut env = setup_environment();

        // 小于测试
        let result = Comparison::eval_less(&[Expr::Number(2), Expr::Number(3)], &mut env);
        assert_eq!(result, Ok(Expr::Symbol("t".to_string())));

        // 相等
        let result = Comparison::eval_less(&[Expr::Number(3), Expr::Number(3)], &mut env);
        assert_eq!(result, Ok(Expr::List(vec![])));

        // 大于
        let result = Comparison::eval_less(&[Expr::Number(5), Expr::Number(3)], &mut env);
        assert_eq!(result, Ok(Expr::List(vec![])));

        // 数字与浮点数
        let result = Comparison::eval_less(&[Expr::Number(3), Expr::Float(3.5)], &mut env);
        assert_eq!(result, Ok(Expr::Symbol("t".to_string())));
    }

    #[test]
    fn test_less_operator_with_floats() {
        let mut env = setup_environment();

        // 浮点数小于测试
        let result = Comparison::eval_less(&[Expr::Float(2.5), Expr::Number(3)], &mut env);
        assert_eq!(result, Ok(Expr::Symbol("t".to_string())));

        // 相等浮点数
        let result = Comparison::eval_less(&[Expr::Float(3.0), Expr::Float(3.0)], &mut env);
        assert_eq!(result, Ok(Expr::List(vec![])));

        // 大于浮点数
        let result = Comparison::eval_less(&[Expr::Float(5.0), Expr::Float(2.6)], &mut env);
        assert_eq!(result, Ok(Expr::List(vec![])));

        // 较小的浮点数比较
        let result = Comparison::eval_less(&[Expr::Float(0.00001), Expr::Float(0.0001)], &mut env);
        assert_eq!(result, Ok(Expr::Symbol("t".to_string())));
    }

    #[test]
    fn test_less_operator_invalid_arguments() {
        let mut env = setup_environment();

        // 非数字类型
        let result = Comparison::eval_less(&[Expr::Symbol("a".to_string()), Expr::Number(3)], &mut env);
        assert_eq!(result, Err(LispError::new("`<` arguments must be numbers")));

        // 不足的参数数量
        let result = Comparison::eval_less(&[Expr::Number(5)], &mut env);
        assert_eq!(result, Err(LispError::new("`<` expects exactly two arguments")));
    }

    #[test]
    fn test_less_equal_operator() {
        let mut env = setup_environment();

        // 小于等于测试
        let result = Comparison::eval_less_equal(&[Expr::Number(2), Expr::Number(3)], &mut env);
        assert_eq!(result, Ok(Expr::Symbol("t".to_string())));

        // 相等
        let result = Comparison::eval_less_equal(&[Expr::Number(3), Expr::Number(3)], &mut env);
        assert_eq!(result, Ok(Expr::Symbol("t".to_string())));

        // 大于
        let result = Comparison::eval_less_equal(&[Expr::Number(5), Expr::Number(3)], &mut env);
        assert_eq!(result, Ok(Expr::List(vec![])));

        // 数字与浮点数
        let result = Comparison::eval_less_equal(&[Expr::Number(3), Expr::Float(3.0)], &mut env);
        assert_eq!(result, Ok(Expr::Symbol("t".to_string())));
    }

    #[test]
    fn test_less_equal_operator_with_floats() {
        let mut env = setup_environment();

        // 浮点数小于等于测试
        let result = Comparison::eval_less_equal(&[Expr::Float(2.5), Expr::Number(3)], &mut env);
        assert_eq!(result, Ok(Expr::Symbol("t".to_string())));

        // 相等浮点数
        let result = Comparison::eval_less_equal(&[Expr::Float(2.6), Expr::Float(2.6)], &mut env);
        assert_eq!(result, Ok(Expr::Symbol("t".to_string())));

        // 大于浮点数
        let result = Comparison::eval_less_equal(&[Expr::Float(5.0), Expr::Float(2.6)], &mut env);
        assert_eq!(result, Ok(Expr::List(vec![])));

        // 小的浮点数与整数
        let result = Comparison::eval_less_equal(&[Expr::Float(0.0001), Expr::Number(1)], &mut env);
        assert_eq!(result, Ok(Expr::Symbol("t".to_string())));
    }

    #[test]
    fn test_less_equal_operator_invalid_arguments() {
        let mut env = setup_environment();

        // 非数字类型
        let result = Comparison::eval_less_equal(&[Expr::Symbol("a".to_string()), Expr::Number(3)], &mut env);
        assert_eq!(result, Err(LispError::new("`<=` arguments must be numbers")));

        // 不足的参数数量
        let result = Comparison::eval_less_equal(&[Expr::Number(5)], &mut env);
        assert_eq!(result, Err(LispError::new("`<=` expects exactly two arguments")));
    }

    #[test]
    fn test_equal_operator() {
        let mut env = setup_environment();

        // 列表不相等，引用不同
        let result = Comparison::eval_equal(&[
            Expr::List(vec![
                Expr::Symbol("quote".to_string()),
                Expr::List(vec![Expr::Number(1), Expr::Number(2), Expr::Number(3)]),
            ]),
            Expr::List(vec![
                Expr::Symbol("quote".to_string()),
                Expr::List(vec![Expr::Number(1), Expr::Number(2), Expr::Number(3)]),
            ]),
        ], &mut env);
        assert_eq!(result, Ok(Expr::List(vec![])));
    
        // 列表相等，引用相同
        let list = Expr::List(vec![
            Expr::Symbol("quote".to_string()),
            Expr::List(vec![Expr::Number(1), Expr::Number(2), Expr::Number(3)]),
        ]);
        env.set_symbol("a".to_string(), list.clone());
        env.set_symbol("b".to_string(), list.clone());
        let result = Comparison::eval_equal(&[Expr::Symbol("a".to_string()), Expr::Symbol("b".to_string())], &mut env);
        assert_eq!(result, Ok(Expr::List(vec![])));
    
        // 符号相等
        let result = Comparison::eval_equal(&[Expr::Symbol("a".to_string()), Expr::Symbol("a".to_string())], &mut env);
        assert_eq!(result, Ok(Expr::List(vec![])));
    
        // 符号不相等
        let result = Comparison::eval_equal(&[Expr::Symbol("a".to_string()), Expr::Symbol("b".to_string())], &mut env);
        assert_eq!(result, Ok(Expr::List(vec![])));
    }
    
    #[test]
    fn test_not_equal_operator() {
        let mut env = setup_environment();
    
        // 列表不相等，引用不同
        let result = Comparison::eval_not_equal(&[
            Expr::List(vec![
                Expr::Symbol("quote".to_string()),
                Expr::List(vec![Expr::Number(1), Expr::Number(2), Expr::Number(3)]),
            ]),
            Expr::List(vec![
                Expr::Symbol("quote".to_string()),
                Expr::List(vec![Expr::Number(1), Expr::Number(2), Expr::Number(3)]),
            ]),
        ], &mut env);
        assert_eq!(result, Ok(Expr::Symbol("t".to_string())));
    
        // 列表相等，引用相同
        let list = Expr::List(vec![Expr::Number(1), Expr::Number(2), Expr::Number(3)]);
        env.set_symbol("a".to_string(), list.clone());
        env.set_symbol("b".to_string(), list.clone());
        let result = Comparison::eval_not_equal(&[Expr::Symbol("a".to_string()), Expr::Symbol("b".to_string())], &mut env);
        assert_eq!(result, Ok(Expr::Symbol("t".to_string())));
    
        // 符号不相等
        let result = Comparison::eval_not_equal(&[Expr::Symbol("a".to_string()), Expr::Symbol("b".to_string())], &mut env);
        assert_eq!(result, Ok(Expr::Symbol("t".to_string())));
    
        // 符号相等
        let result = Comparison::eval_not_equal(&[Expr::Symbol("a".to_string()), Expr::Symbol("a".to_string())], &mut env);
        assert_eq!(result, Ok(Expr::Symbol("t".to_string())));
    }    

    #[test]
    fn test_equal_operator_with_floats() {
        let mut env = setup_environment();

        // 浮点数相等
        let result = Comparison::eval_equal(&[Expr::Float(3.0), Expr::Float(3.0)], &mut env);
        assert_eq!(result, Ok(Expr::Symbol("t".to_string())));

        // 浮点数不相等
        let result = Comparison::eval_equal(&[Expr::Float(3.0), Expr::Float(3.1)], &mut env);
        assert_eq!(result, Ok(Expr::List(vec![])));

        // 浮点数与整数
        let result = Comparison::eval_equal(&[Expr::Float(3.0), Expr::Number(3)], &mut env);
        assert_eq!(result, Ok(Expr::Symbol("t".to_string())));

        // 浮点数与整数
        let result = Comparison::eval_equal(&[Expr::Number(3), Expr::Float(3.00000001)], &mut env);
        assert_eq!(result, Ok(Expr::List(vec![])));

        // 整数与整数
        let result = Comparison::eval_equal(&[Expr::Number(3), Expr::Number(3)], &mut env);
        assert_eq!(result, Ok(Expr::Symbol("t".to_string())));

        // 整数与整数
        let result = Comparison::eval_equal(&[Expr::Number(3), Expr::Number(4)], &mut env);
        assert_eq!(result, Ok(Expr::List(vec![])));
    }

    #[test]
    fn test_equal_operator_with_symbols() {
        let mut env = setup_environment();

        // 符号相等
        let result = Comparison::eval_equal(&[Expr::Symbol("a".to_string()), Expr::Symbol("a".to_string())], &mut env);
        assert_eq!(result, Ok(Expr::Symbol("t".to_string())));

        // 符号不相等
        let result = Comparison::eval_equal(&[Expr::Symbol("a".to_string()), Expr::Symbol("b".to_string())], &mut env);
        assert_eq!(result, Ok(Expr::List(vec![])));

        // 符号与数字
        let result = Comparison::eval_equal(&[Expr::Symbol("a".to_string()), Expr::Number(3)], &mut env);
        assert_eq!(result, Ok(Expr::List(vec![])));
    }

    #[test]
    fn test_not_equal_operator_with_floats() {
        let mut env = setup_environment();

        // 浮点数不相等
        let result = Comparison::eval_not_equal(&[Expr::Float(3.0), Expr::Float(3.1)], &mut env);
        assert_eq!(result, Ok(Expr::Symbol("t".to_string())));

        // 浮点数相等
        let result = Comparison::eval_not_equal(&[Expr::Float(3.0), Expr::Float(3.0)], &mut env);
        assert_eq!(result, Ok(Expr::List(vec![])));

        // 浮点数与整数
        let result = Comparison::eval_not_equal(&[Expr::Float(3.0), Expr::Number(3)], &mut env);
        assert_eq!(result, Ok(Expr::List(vec![])));

        // 浮点数与整数
        let result = Comparison::eval_not_equal(&[Expr::Number(3), Expr::Float(3.00000001)], &mut env);
        assert_eq!(result, Ok(Expr::Symbol("t".to_string())));

        // 整数与整数
        let result = Comparison::eval_not_equal(&[Expr::Number(3), Expr::Number(3)], &mut env);
        assert_eq!(result, Ok(Expr::List(vec![])));

        // 整数与整数
        let result = Comparison::eval_not_equal(&[Expr::Number(3), Expr::Number(4)], &mut env);
        assert_eq!(result, Ok(Expr::Symbol("t".to_string())));
    }

    #[test]
    fn test_not_equal_operator_with_symbols() {
        let mut env = setup_environment();

        // 符号相等
        let result = Comparison::eval_not_equal(&[Expr::Symbol("a".to_string()), Expr::Symbol("a".to_string())], &mut env);
        assert_eq!(result, Ok(Expr::List(vec![])));

        // 符号不相等
        let result = Comparison::eval_not_equal(&[Expr::Symbol("a".to_string()), Expr::Symbol("b".to_string())], &mut env);
        assert_eq!(result, Ok(Expr::Symbol("t".to_string())));

        // 符号与数字
        let result = Comparison::eval_not_equal(&[Expr::Symbol("a".to_string()), Expr::Number(3)], &mut env);
        assert_eq!(result, Ok(Expr::Symbol("t".to_string())));
    }
}
