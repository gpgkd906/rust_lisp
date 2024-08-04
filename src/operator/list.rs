// operator/list.rs

use crate::environment::Environment;
use crate::exception::LispError;
use crate::expression::Expr;
use crate::evaluator::Evaluator;

pub struct ListOps;

impl ListOps {
    pub fn eval_car(args: &[Expr], env: &mut Environment) -> Result<Expr, LispError> {
        if args.len() != 1 {
            return Err(LispError::new("car requires exactly one argument"));
        }

        let list_expr = Evaluator::eval_tree(&args[0], env)?;
        if let Expr::List(list) = list_expr {
            if let Some(first) = list.first() {
                return Ok(first.clone());
            } else {
                return Ok(Expr::List(vec![])); // return empty list
            }
        }
        Err(LispError::new("car: argument is not a list"))
    }

    pub fn eval_cdr(args: &[Expr], env: &mut Environment) -> Result<Expr, LispError> {
        if args.len() != 1 {
            return Err(LispError::new("cdr requires exactly one argument"));
        }

        let list_expr = Evaluator::eval_tree(&args[0], env)?;
        if let Expr::List(list) = list_expr {
            if list.len() > 1 {
                return Ok(Expr::List(list[1..].to_vec()));
            } else {
                return Ok(Expr::List(vec![])); // return empty list
            }
        }
        Err(LispError::new("cdr: argument is not a list"))
    }

    pub fn eval_cons(args: &[Expr], env: &mut Environment) -> Result<Expr, LispError> {
        if args.len() != 2 {
            return Err(LispError::new("cons requires exactly two arguments"));
        }

        let first = Evaluator::eval_tree(&args[0], env)?;
        let rest = Evaluator::eval_tree(&args[1], env)?;

        let mut list = match rest {
            Expr::List(lst) => lst,
            _ => vec![rest],
        };

        list.insert(0, first);

        Ok(Expr::List(list))
    }
    
    pub fn eval_length(args: &[Expr], env: &mut Environment) -> Result<Expr, LispError> {
        if args.len() != 1 {
            return Err(LispError::new("length requires exactly one argument"));
        }

        let list_expr = Evaluator::eval_tree(&args[0], env)?;
        if let Expr::List(list) = list_expr {
            return Ok(Expr::Number(list.len() as i64));
        }
        Err(LispError::new("length: argument is not a list"))
    }

    pub fn eval_quote(args: &[Expr], _env: &mut Environment) -> Result<Expr, LispError> {
        if args.len() != 1 {
            return Err(LispError::new("quote requires exactly one argument"));
        }
        Ok(args[0].clone())
    }
}
