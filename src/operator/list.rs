// operator/list.rs

use crate::environment::Environment;
use crate::exception::LispError;
use crate::expression::Expr;
use crate::evaluator::Evaluator;

pub struct ListOps;

impl ListOps {
    pub fn eval_cons(args: &[Expr], env: &mut Environment) -> Result<Expr, LispError> {
        if args.len() != 2 {
            return Err(LispError::new("cons requires exactly two arguments"));
        }

        let first = Evaluator::eval_tree(&args[0], env)?;
        let second = Evaluator::eval_tree(&args[1], env)?;

        match second {
            Expr::List(mut list) => {
                list.insert(0, first);
                Ok(Expr::List(list))
            }
            _ => Err(LispError::new("cons: second argument must be a list")),
        }
    }

    pub fn eval_car(args: &[Expr], env: &mut Environment) -> Result<Expr, LispError> {
        if args.len() != 1 {
            return Err(LispError::new("car requires exactly one argument"));
        }

        let list = Evaluator::eval_tree(&args[0], env)?;

        match list {
            Expr::List(ref list) if !list.is_empty() => Ok(list[0].clone()),
            Expr::List(_) => Ok(Expr::List(vec![])),  // 返回空列表而不是错误
            _ => Err(LispError::new("car: argument must be a list")),
        }
    }

    pub fn eval_cdr(args: &[Expr], env: &mut Environment) -> Result<Expr, LispError> {
        if args.len() != 1 {
            return Err(LispError::new("cdr requires exactly one argument"));
        }

        let list = Evaluator::eval_tree(&args[0], env)?;

        match list {
            Expr::List(ref list) if list.len() > 1 => Ok(Expr::List(list[1..].to_vec())),
            Expr::List(_) => Ok(Expr::List(vec![])),  // 返回空列表而不是错误
            _ => Err(LispError::new("cdr: argument must be a list")),
        }
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


#[cfg(test)]
mod tests {
    use super::*;
    use crate::environment::Environment;
    use crate::expression::Expr;

    // Helper function to set up the environment
    fn setup_environment() -> Environment {
        let mut env = Environment::initialize();
        env.set_symbol("nil".to_string(), Expr::List(vec![]));
        env.set_symbol("t".to_string(), Expr::Symbol("t".to_string()));
        env
    }

    #[test]
    fn test_eval_cons_success() {
        let mut env = setup_environment();
        
        // 使用 quote 确保 cons 的第二个参数是列表
        let expr = Expr::List(vec![
            Expr::Symbol("cons".to_string()),
            Expr::Number(1),
            Expr::List(vec![
                Expr::Symbol("quote".to_string()),
                Expr::List(vec![Expr::Number(2), Expr::Number(3)]),
            ]),
        ]);
    
        let result = Evaluator::eval_tree(&expr, &mut env);
        assert_eq!(
            result,
            Ok(Expr::List(vec![Expr::Number(1), Expr::Number(2), Expr::Number(3)]))
        );
    }

    #[test]
    fn test_eval_cons_invalid_second_argument() {
        let mut env = setup_environment();
        
        // cons with a non-list second argument should fail
        let result = ListOps::eval_cons(
            &[
                Expr::Number(1),
                Expr::Number(2), // not a list
            ],
            &mut env,
        );
        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err.to_string(), "cons: second argument must be a list");
        }
    }

    #[test]
    fn test_eval_car_success() {
        let mut env = setup_environment();
        
        // 使用 quote 确保 car 的参数是一个列表
        let expr = Expr::List(vec![
            Expr::Symbol("car".to_string()),
            Expr::List(vec![
                Expr::Symbol("quote".to_string()),
                Expr::List(vec![Expr::Number(1), Expr::Number(2), Expr::Number(3)]),
            ]),
        ]);
        let result = Evaluator::eval_tree(&expr, &mut env);
        assert_eq!(result, Ok(Expr::Number(1)));
    }

    #[test]
    fn test_eval_car_empty_list() {
        let mut env = setup_environment();
        
        // car of empty list should return an empty list
        let result = ListOps::eval_car(&[Expr::List(vec![])], &mut env);
        assert_eq!(result, Ok(Expr::List(vec![])));
    }

    #[test]
    fn test_eval_cdr_success() {
        let mut env = setup_environment();
        
        // 使用 quote 确保 cdr 的参数是一个列表
        let expr = Expr::List(vec![
            Expr::Symbol("cdr".to_string()),
            Expr::List(vec![
                Expr::Symbol("quote".to_string()),
                Expr::List(vec![Expr::Number(1), Expr::Number(2), Expr::Number(3)]),
            ]),
        ]);
        let result = Evaluator::eval_tree(&expr, &mut env);
        assert_eq!(result, Ok(Expr::List(vec![Expr::Number(2), Expr::Number(3)])));
    }

    #[test]
    fn test_eval_cdr_empty_list() {
        let mut env = setup_environment();
        
        // cdr of empty list should return an empty list
        let result = ListOps::eval_cdr(&[Expr::List(vec![])], &mut env);
        assert_eq!(result, Ok(Expr::List(vec![])));
    }
}
