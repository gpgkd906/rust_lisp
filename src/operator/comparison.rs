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
            return Err(LispError::new("= requires exactly 2 arguments"));
        }

        let left = Evaluator::eval_tree(&args[0], env)?;
        let right = Evaluator::eval_tree(&args[1], env)?;

        let result = match (left, right) {
            (Expr::Number(a), Expr::Number(b)) => a == b,
            (Expr::Symbol(a), Expr::Symbol(b)) => a == b,
            (Expr::Str(a), Expr::Str(b)) => a == b,
            (Expr::List(a), Expr::List(b)) => a == b,
            _ => false,
        };

        if result {
            Ok(Expr::Symbol("T".to_string()))
        } else {
            Ok(Expr::List(vec![])) // 返回 NIL (空列表) 表示不等
        }
    }
}
