// operator/arithmetic.rs

use crate::operator::OperatorRegistry;
use crate::environment::Environment;
use crate::exception::LispError;
use crate::expression::Expr;
use crate::evaluator::Evaluator;

pub struct Arithmetic;

impl Arithmetic {
    pub fn eval_add(args: &[Expr], env: &mut Environment) -> Result<Expr, LispError> {
        let mut sum = 0.0;
        let mut has_float = false;

        for arg in args {
            match Evaluator::eval(arg, env)? {
                Expr::Number(n) => sum += n as f64,
                Expr::Float(f) => {
                    sum += f;
                    has_float = true;
                },
                _ => return Err(LispError::new("Invalid number")),
            }
        }

        if has_float || sum.fract() != 0.0 {
            Ok(Expr::Float(sum))
        } else {
            Ok(Expr::Number(sum as i64))
        }
    }

    pub fn eval_subtract(args: &[Expr], env: &mut Environment) -> Result<Expr, LispError> {
        let mut iter = args.iter();
        let first = iter
            .next()
            .ok_or_else(|| LispError::new("Subtraction requires at least one argument"))?;
        let mut result = match Evaluator::eval(first, env)? {
            Expr::Number(n) => n as f64,
            Expr::Float(f) => f,
            _ => return Err(LispError::new("Invalid number")),
        };
        let mut has_float = matches!(first, Expr::Float(_));

        for arg in iter {
            match Evaluator::eval(arg, env)? {
                Expr::Number(n) => result -= n as f64,
                Expr::Float(f) => {
                    result -= f;
                    has_float = true;
                },
                _ => return Err(LispError::new("Invalid number")),
            }
        }

        if has_float || result.fract() != 0.0 {
            Ok(Expr::Float(result))
        } else {
            Ok(Expr::Number(result as i64))
        }
    }

    pub fn eval_multiply(args: &[Expr], env: &mut Environment) -> Result<Expr, LispError> {
        let mut product = 1.0;
        let mut has_float = false;

        for arg in args {
            match Evaluator::eval(arg, env)? {
                Expr::Number(n) => product *= n as f64,
                Expr::Float(f) => {
                    product *= f;
                    has_float = true;
                },
                _ => return Err(LispError::new("Invalid number")),
            }
        }

        if has_float || product.fract() != 0.0 {
            Ok(Expr::Float(product))
        } else {
            Ok(Expr::Number(product as i64))
        }
    }

    pub fn eval_divide(args: &[Expr], env: &mut Environment) -> Result<Expr, LispError> {
        let mut iter = args.iter();
        let first = iter
            .next()
            .ok_or_else(|| LispError::new("Division requires at least one argument"))?;
        let mut result = match Evaluator::eval(first, env)? {
            Expr::Number(n) => n as f64,
            Expr::Float(f) => f,
            _ => return Err(LispError::new("Invalid number")),
        };
        let mut has_float = matches!(first, Expr::Float(_));

        for arg in iter {
            match Evaluator::eval(arg, env)? {
                Expr::Number(n) => {
                    if n == 0 {
                        return Err(LispError::new("Division by zero"));
                    }
                    result /= n as f64;
                }
                Expr::Float(f) => {
                    if f == 0.0 {
                        return Err(LispError::new("Division by zero"));
                    }
                    result /= f;
                    has_float = true;
                }
                _ => return Err(LispError::new("Invalid number")),
            }
        }

        if has_float || result.fract() != 0.0 {
            Ok(Expr::Float(result))
        } else {
            Ok(Expr::Number(result as i64))
        }
    }
}

pub fn register_arithmetic_operators() {
    OperatorRegistry::register("+", Arithmetic::eval_add);
    OperatorRegistry::register("-", Arithmetic::eval_subtract);
    OperatorRegistry::register("*", Arithmetic::eval_multiply);
    OperatorRegistry::register("/", Arithmetic::eval_divide);
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
    fn test_eval_add_with_floats() {
        let mut env = Environment::initialize();
        let args = vec![Expr::Float(2.5), Expr::Number(3), Expr::Float(4.5)];
        let result = Arithmetic::eval_add(&args, &mut env);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Expr::Float(10.0));
    }

    #[test]
    fn test_eval_add_no_arguments() {
        let mut env = Environment::initialize();
        let args: Vec<Expr> = vec![];
        let result = Arithmetic::eval_add(&args, &mut env);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Expr::Number(0)); // Adding no arguments should return zero
    }

    #[test]
    fn test_eval_add_single_argument() {
        let mut env = Environment::initialize();
        let args = vec![Expr::Number(42)];
        let result = Arithmetic::eval_add(&args, &mut env);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Expr::Number(42)); // Adding a single argument should return the argument itself
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
    fn test_eval_sub_with_floats() {
        let mut env = Environment::initialize();
        let args = vec![Expr::Float(10.5), Expr::Float(3.2), Expr::Number(2)];
        let result = Arithmetic::eval_subtract(&args, &mut env);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Expr::Float(5.3));
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
    fn test_eval_sub_single_argument() {
        let mut env = Environment::initialize();
        let args = vec![Expr::Number(5)];
        let result = Arithmetic::eval_subtract(&args, &mut env);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Expr::Number(5)); // Subtraction with a single argument returns the argument itself
    }

    #[test]
    fn test_eval_sub_negative_result() {
        let mut env = Environment::initialize();
        let args = vec![Expr::Number(5), Expr::Number(10)];
        let result = Arithmetic::eval_subtract(&args, &mut env);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Expr::Number(-5)); // Should return a negative number
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
    fn test_eval_mul_with_floats() {
        let mut env = Environment::initialize();
        let args = vec![Expr::Float(2.5), Expr::Number(3), Expr::Float(4.0)];
        let result = Arithmetic::eval_multiply(&args, &mut env);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Expr::Float(30.0));
    }

    #[test]
    fn test_eval_mul_no_arguments() {
        let mut env = Environment::initialize();
        let args: Vec<Expr> = vec![];
        let result = Arithmetic::eval_multiply(&args, &mut env);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Expr::Number(1)); // Multiplying no arguments should return one
    }

    #[test]
    fn test_eval_mul_single_argument() {
        let mut env = Environment::initialize();
        let args = vec![Expr::Number(42)];
        let result = Arithmetic::eval_multiply(&args, &mut env);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Expr::Number(42)); // Multiplying a single argument should return the argument itself
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
    fn test_eval_div_with_floats() {
        let mut env = Environment::initialize();
        let args = vec![Expr::Float(10.0), Expr::Number(4)];
        let result = Arithmetic::eval_divide(&args, &mut env);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Expr::Float(2.5));
    }

    #[test]
    fn test_eval_div_with_floats_result_integer() {
        let mut env = Environment::initialize();
        let args = vec![Expr::Float(10.0), Expr::Float(2.0)];
        let result = Arithmetic::eval_divide(&args, &mut env);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Expr::Float(5.0));
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
    fn test_eval_div_by_zero_float() {
        let mut env = Environment::initialize();
        let args = vec![Expr::Float(10.0), Expr::Float(0.0)];
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

        // Test with no arguments
        let args: Vec<Expr> = vec![];
        let result = Arithmetic::eval_divide(&args, &mut env);
        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err.to_string(), "Division requires at least one argument");
        }

        // Test with one argument
        let args = vec![Expr::Number(10)];
        let result = Arithmetic::eval_divide(&args, &mut env);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Expr::Number(10)); // Division with a single argument returns the argument itself
    }

    #[test]
    fn test_eval_div_nested_division_with_floats() {
        let mut env = Environment::initialize();

        // Construct expression (/ 100 (/ 10 2))
        let expr = Expr::List(vec![
            Expr::Symbol("/".to_string()),
            Expr::Number(100),
            Expr::List(vec![
                Expr::Symbol("/".to_string()),
                Expr::Number(10),
                Expr::Number(2),
            ]),
        ]);

        let result = Evaluator::eval(&expr, &mut env);
        assert_eq!(result, Ok(Expr::Number(20))); // Should return 20
    }

    #[test]
    fn test_eval_nested_multiplication_with_float() {
        let mut env = Environment::initialize();
        
        // Construct expression (* 100 5 (/ 3 2))
        let expr = Expr::List(vec![
            Expr::Symbol("*".to_string()),
            Expr::Number(100),
            Expr::Number(5),
            Expr::List(vec![
                Expr::Symbol("/".to_string()),
                Expr::Number(3),
                Expr::Number(2),
            ]),
        ]);

        let result = Evaluator::eval(&expr, &mut env);
        assert_eq!(result, Ok(Expr::Float(750.0))); // Should return 750.0
    }

    #[test]
    fn test_eval_large_numbers() {
        let mut env = Environment::initialize();
        let args = vec![Expr::Number(1_000_000_000), Expr::Number(2_000_000_000)];
        let result = Arithmetic::eval_add(&args, &mut env);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Expr::Number(3_000_000_000));
    }
}
