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
