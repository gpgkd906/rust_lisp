// operator/arithmetic.rs

use crate::environment::Environment;
use crate::exception::LispError;
use crate::expression::Expr;
use crate::evaluator::Evaluator;

pub struct Arithmetic;

impl Arithmetic {
    pub fn eval_add(args: &[Expr], env: &mut Environment) -> Result<Expr, LispError> {
        let sum: i64 = args
            .iter()
            .map(|arg| Evaluator::eval_tree(arg, env))
            .map(|result| result.and_then(|expr| match expr {
                Expr::Number(n) => Ok(n),
                _ => Err(LispError::new("Invalid number")),
            }))
            .sum::<Result<i64, LispError>>()?;
        Ok(Expr::Number(sum))
    }

    pub fn eval_subtract(args: &[Expr], env: &mut Environment) -> Result<Expr, LispError> {
        let mut iter = args.iter();
        let first = iter
            .next()
            .ok_or_else(|| LispError::new("Subtraction requires at least one argument"))?;
        let mut result = match Evaluator::eval_tree(first, env)? {
            Expr::Number(n) => n,
            _ => return Err(LispError::new("Invalid number")),
        };
        for arg in iter {
            let value = match Evaluator::eval_tree(arg, env)? {
                Expr::Number(n) => n,
                _ => return Err(LispError::new("Invalid number")),
            };
            result -= value;
        }
        Ok(Expr::Number(result))
    }

    pub fn eval_multiply(args: &[Expr], env: &mut Environment) -> Result<Expr, LispError> {
        let product: i64 = args
            .iter()
            .map(|arg| Evaluator::eval_tree(arg, env))
            .map(|result| result.and_then(|expr| match expr {
                Expr::Number(n) => Ok(n),
                _ => Err(LispError::new("Invalid number")),
            }))
            .product::<Result<i64, LispError>>()?;
        Ok(Expr::Number(product))
    }

    pub fn eval_divide(args: &[Expr], env: &mut Environment) -> Result<Expr, LispError> {
        let mut iter = args.iter();
        let first = iter
            .next()
            .ok_or_else(|| LispError::new("Division requires at least one argument"))?;
        let mut result = match Evaluator::eval_tree(first, env)? {
            Expr::Number(n) => n,
            _ => return Err(LispError::new("Invalid number")),
        };
        for arg in iter {
            let value = match Evaluator::eval_tree(arg, env)? {
                Expr::Number(n) => n,
                _ => return Err(LispError::new("Invalid number")),
            };
            if value == 0 {
                return Err(LispError::new("Division by zero"));
            }
            result /= value;
        }
        Ok(Expr::Number(result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::environment::Environment;
    use crate::expression::Expr;

    #[test]
    fn test_eval_add_success() {
        let mut env = Environment::initialize();
        let args = vec![Expr::Number(2), Expr::Number(3), Expr::Number(4)];
        let result = Arithmetic::eval_add(&args, &mut env);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Expr::Number(9));
    }

    #[test]
    fn test_eval_add_non_number_argument() {
        let mut env = Environment::initialize();
        let args = vec![Expr::Number(2), Expr::Symbol("a".to_string())];
        let result = Arithmetic::eval_add(&args, &mut env);

        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err.to_string(), "Undefined symbol: a");
        }
    }

    #[test]
    fn test_eval_sub_success() {
        let mut env = Environment::initialize();
        let args = vec![Expr::Number(10), Expr::Number(3), Expr::Number(2)];
        let result = Arithmetic::eval_subtract(&args, &mut env);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Expr::Number(5));
    }

    #[test]
    fn test_eval_sub_no_arguments() {
        let mut env = Environment::initialize();
        let args: Vec<Expr> = vec![];
        let result = Arithmetic::eval_subtract(&args, &mut env);

        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err.to_string(), "Subtraction requires at least one argument");
        }
    }

    #[test]
    fn test_eval_sub_non_number_argument() {
        let mut env = Environment::initialize();
        let args = vec![Expr::Number(10), Expr::Symbol("a".to_string())];
        let result = Arithmetic::eval_subtract(&args, &mut env);

        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err.to_string(), "Undefined symbol: a");
        }
    }

    #[test]
    fn test_eval_mul_success() {
        let mut env = Environment::initialize();
        let args = vec![Expr::Number(2), Expr::Number(3), Expr::Number(4)];
        let result = Arithmetic::eval_multiply(&args, &mut env);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Expr::Number(24));
    }

    #[test]
    fn test_eval_mul_non_number_argument() {
        let mut env = Environment::initialize();
        let args = vec![Expr::Number(2), Expr::Symbol("a".to_string())];
        let result = Arithmetic::eval_multiply(&args, &mut env);

        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err.to_string(), "Undefined symbol: a");
        }
    }

    #[test]
    fn test_eval_div_success() {
        let mut env = Environment::initialize();
        let args = vec![Expr::Number(10), Expr::Number(2)];
        let result = Arithmetic::eval_divide(&args, &mut env);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Expr::Number(5));
    }

    #[test]
    fn test_eval_div_by_zero() {
        let mut env = Environment::initialize();
        let args = vec![Expr::Number(10), Expr::Number(0)];
        let result = Arithmetic::eval_divide(&args, &mut env);

        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err.to_string(), "Division by zero");
        }
    }

    #[test]
    fn test_eval_div_non_number_argument() {
        let mut env = Environment::initialize();
        let args = vec![Expr::Number(10), Expr::Symbol("a".to_string())];
        let result = Arithmetic::eval_divide(&args, &mut env);

        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err.to_string(), "Undefined symbol: a");
        }
    }

    #[test]
    fn test_eval_div_incorrect_number_of_arguments() {
        let mut env = Environment::initialize();

        // 测试少于一个参数
        let args: Vec<Expr> = vec![];
        let result = Arithmetic::eval_divide(&args, &mut env);
        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err.to_string(), "Division requires at least one argument");
        }
    }
}
