// operator/comparison.rs

use crate::environment::Environment;
use crate::exception::LispError;
use crate::expression::Expr;
use crate::evaluator::Evaluator;

pub struct Comparison;

impl Comparison {
    pub fn eval_greater(args: &[Expr], env: &mut Environment) -> Result<Expr, LispError> {
        if args.len() != 2 {
            return Err(LispError::new("> requires exactly two arguments"));
        }

        let left = Evaluator::eval_tree(&args[0], env)?;
        let right = Evaluator::eval_tree(&args[1], env)?;

        match (left, right) {
            (Expr::Number(l), Expr::Number(r)) => Ok(Expr::Str((l > r).to_string())),
            _ => Err(LispError::new("Invalid comparison")),
        }
    }

    pub fn eval_less(args: &[Expr], env: &mut Environment) -> Result<Expr, LispError> {
        if args.len() != 2 {
            return Err(LispError::new("< requires exactly two arguments"));
        }

        let left = Evaluator::eval_tree(&args[0], env)?;
        let right = Evaluator::eval_tree(&args[1], env)?;

        match (left, right) {
            (Expr::Number(l), Expr::Number(r)) => Ok(Expr::Str((l < r).to_string())),
            _ => Err(LispError::new("Invalid comparison")),
        }
    }

    pub fn eval_greater_equal(args: &[Expr], env: &mut Environment) -> Result<Expr, LispError> {
        if args.len() != 2 {
            return Err(LispError::new(">= requires exactly two arguments"));
        }

        let left = Evaluator::eval_tree(&args[0], env)?;
        let right = Evaluator::eval_tree(&args[1], env)?;

        match (left, right) {
            (Expr::Number(l), Expr::Number(r)) => Ok(Expr::Str((l >= r).to_string())),
            _ => Err(LispError::new("Invalid comparison")),
        }
    }

    pub fn eval_less_equal(args: &[Expr], env: &mut Environment) -> Result<Expr, LispError> {
        if args.len() != 2 {
            return Err(LispError::new("<= requires exactly two arguments"));
        }

        let left = Evaluator::eval_tree(&args[0], env)?;
        let right = Evaluator::eval_tree(&args[1], env)?;

        match (left, right) {
            (Expr::Number(l), Expr::Number(r)) => Ok(Expr::Str((l <= r).to_string())),
            _ => Err(LispError::new("Invalid comparison")),
        }
    }

    pub fn eval_equal(args: &[Expr], env: &mut Environment) -> Result<Expr, LispError> {
        if args.len() != 2 {
            return Err(LispError::new("= requires exactly two arguments"));
        }

        let left = Evaluator::eval_tree(&args[0], env)?;
        let right = Evaluator::eval_tree(&args[1], env)?;

        match (left, right) {
            (Expr::Number(l), Expr::Number(r)) => Ok(Expr::Str((l == r).to_string())),
            _ => Err(LispError::new("Invalid comparison")),
        }
    }
}
