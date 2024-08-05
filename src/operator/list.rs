// operator/list.rs
use crate::operator::OperatorRegistry;
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
    
        let first = Evaluator::eval(&args[0], env)?;
        let second = Evaluator::eval(&args[1], env)?;
    
        match second {
            Expr::List(mut list) => {
                list.insert(0, first);
                Ok(Expr::List(list))
            }
            _ => Ok(Expr::DottedPair(Box::new(first), Box::new(second))),
        }
    }
    
    pub fn eval_car(args: &[Expr], env: &mut Environment) -> Result<Expr, LispError> {
        if args.len() != 1 {
            return Err(LispError::new("car requires exactly one argument"));
        }

        let list = Evaluator::eval(&args[0], env)?;

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

        let list = Evaluator::eval(&args[0], env)?;

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

        let list_expr = Evaluator::eval(&args[0], env)?;
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

pub fn register_list_operators() {
    OperatorRegistry::register("cons", ListOps::eval_cons);
    OperatorRegistry::register("car", ListOps::eval_car);
    OperatorRegistry::register("cdr", ListOps::eval_cdr);
    OperatorRegistry::register("length", ListOps::eval_length);
    OperatorRegistry::register("quote", ListOps::eval_quote);
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
    
        let result = Evaluator::eval(&expr, &mut env);
        assert_eq!(
            result,
            Ok(Expr::List(vec![Expr::Number(1), Expr::Number(2), Expr::Number(3)]))
        );
    }

    #[test]
    fn test_eval_cons_invalid_second_argument() {
        let mut env = setup_environment();
        
        // cons with a non-list second argument should create a dotted pair
        let expr = Expr::List(vec![
            Expr::Symbol("cons".to_string()),
            Expr::Number(1),
            Expr::Number(2), // not a list
        ]);
        
        let result = Evaluator::eval(&expr, &mut env);
        assert_eq!(
            result,
            Ok(Expr::DottedPair(Box::new(Expr::Number(1)), Box::new(Expr::Number(2))))
        );
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
        let result = Evaluator::eval(&expr, &mut env);
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
        let result = Evaluator::eval(&expr, &mut env);
        assert_eq!(result, Ok(Expr::List(vec![Expr::Number(2), Expr::Number(3)])));
    }

    #[test]
    fn test_eval_cdr_empty_list() {
        let mut env = setup_environment();
        
        // cdr of empty list should return an empty list
        let result = ListOps::eval_cdr(&[Expr::List(vec![])], &mut env);
        assert_eq!(result, Ok(Expr::List(vec![])));
    }

    #[test]
    fn test_eval_length_success() {
        let mut env = setup_environment();
    
        // 使用 quote 确保 length 的参数是一个列表
        let expr = Expr::List(vec![
            Expr::Symbol("length".to_string()),
            Expr::List(vec![
                Expr::Symbol("quote".to_string()),
                Expr::List(vec![Expr::Number(1), Expr::Number(2), Expr::Number(3)]),
            ]),
        ]);
        let result = Evaluator::eval(&expr, &mut env);
        assert_eq!(result, Ok(Expr::Number(3)));
    }
    
    #[test]
    fn test_eval_length_empty_list() {
        let mut env = setup_environment();
    
        // 空列表
        let expr = Expr::List(vec![
            Expr::Symbol("length".to_string()),
            Expr::List(vec![
                Expr::Symbol("quote".to_string()),
                Expr::List(vec![]),
            ]),
        ]);
        let result = Evaluator::eval(&expr, &mut env);
        assert_eq!(result, Ok(Expr::Number(0)));
    }
    
    #[test]
    fn test_eval_length_invalid_argument() {
        let mut env = setup_environment();
    
        // 非列表参数
        let expr = Expr::List(vec![
            Expr::Symbol("length".to_string()),
            Expr::Number(123),
        ]);
        let result = Evaluator::eval(&expr, &mut env);
        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err.to_string(), "length: argument is not a list");
        }
    }

    #[test]
    fn test_eval_quote_success() {
        let mut env = setup_environment();
    
        // 直接返回未评估的表达式
        let expr = Expr::List(vec![
            Expr::Symbol("quote".to_string()),
            Expr::Number(123),
        ]);
        let result = Evaluator::eval(&expr, &mut env);
        assert_eq!(result, Ok(Expr::Number(123)));
    }
    
    #[test]
    fn test_eval_quote_list() {
        let mut env = setup_environment();
    
        // 列表的 quote 测试
        let expr = Expr::List(vec![
            Expr::Symbol("quote".to_string()),
            Expr::List(vec![Expr::Number(1), Expr::Number(2), Expr::Number(3)]),
        ]);
        let result = Evaluator::eval(&expr, &mut env);
        assert_eq!(result, Ok(Expr::List(vec![Expr::Number(1), Expr::Number(2), Expr::Number(3)])));
    }
    
    #[test]
    fn test_eval_cons_with_non_list_second_argument() {
        let mut env = setup_environment();
        
        // cons with a non-list second argument should create a dotted pair
        let expr = Expr::List(vec![
            Expr::Symbol("cons".to_string()),
            Expr::Number(1),
            Expr::Number(2), // not a list
        ]);
        
        let result = Evaluator::eval(&expr, &mut env);
        assert_eq!(
            result,
            Ok(Expr::DottedPair(Box::new(Expr::Number(1)), Box::new(Expr::Number(2))))
        );
    }    
    
    #[test]
    fn test_eval_car_with_non_list_argument() {
        let mut env = setup_environment();
    
        // 参数为非列表
        let expr = Expr::List(vec![
            Expr::Symbol("car".to_string()),
            Expr::Number(123), // 非列表
        ]);
        let result = Evaluator::eval(&expr, &mut env);
        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err.to_string(), "car: argument must be a list");
        }
    }
    
    #[test]
    fn test_eval_cdr_with_non_list_argument() {
        let mut env = setup_environment();
    
        // 参数为非列表
        let expr = Expr::List(vec![
            Expr::Symbol("cdr".to_string()),
            Expr::Number(123), // 非列表
        ]);
        let result = Evaluator::eval(&expr, &mut env);
        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err.to_string(), "cdr: argument must be a list");
        }
    }

    #[test]
    fn test_cons_with_list() {
        let mut env = Environment::initialize();
        let a = Expr::List(vec![Expr::Number(1), Expr::Number(2), Expr::Number(3)]);
        env.set_symbol("a".to_string(), a.clone());
        
        let expr = Expr::List(vec![
            Expr::Symbol("cons".to_string()),
            Expr::Number(4),
            Expr::Symbol("a".to_string()),
        ]);
        
        let result = Evaluator::eval(&expr, &mut env).unwrap();
        assert_eq!(
            result,
            Expr::List(vec![Expr::Number(4), Expr::Number(1), Expr::Number(2), Expr::Number(3)])
        );
    }

    #[test]
    fn test_cons_with_dotted_pair() {
        let mut env = Environment::initialize();
        let a = Expr::List(vec![Expr::Number(1), Expr::Number(2), Expr::Number(3)]);
        env.set_symbol("a".to_string(), a.clone());
        
        let expr = Expr::List(vec![
            Expr::Symbol("cons".to_string()),
            Expr::Symbol("a".to_string()),
            Expr::Number(4),
        ]);
        
        let result = Evaluator::eval(&expr, &mut env).unwrap();
        assert_eq!(
            result,
            Expr::DottedPair(Box::new(a), Box::new(Expr::Number(4)))  // Removed .clone()
        );
    }

    #[test]
    fn test_setf_and_cons() {
        let mut env = Environment::initialize();
        
        // [1] (setf a '(1 2 3))
        let setf_expr = Expr::List(vec![
            Expr::Symbol("setf".to_string()),
            Expr::Symbol("a".to_string()),
            Expr::List(vec![
                Expr::Symbol("quote".to_string()), // 使用 quote 保证列表
                Expr::List(vec![Expr::Number(1), Expr::Number(2), Expr::Number(3)])
            ]),
        ]);
        let result = Evaluator::eval(&setf_expr, &mut env).unwrap();
        assert_eq!(
            result,
            Expr::List(vec![Expr::Number(1), Expr::Number(2), Expr::Number(3)])
        );
    
        // [2] (cons 4 a)
        let cons_expr1 = Expr::List(vec![
            Expr::Symbol("cons".to_string()),
            Expr::Number(4),
            Expr::Symbol("a".to_string()),
        ]);
        let result1 = Evaluator::eval(&cons_expr1, &mut env).unwrap();
        assert_eq!(
            result1,
            Expr::List(vec![Expr::Number(4), Expr::Number(1), Expr::Number(2), Expr::Number(3)])
        );
    
        // [3] (cons a 4)
        let cons_expr2 = Expr::List(vec![
            Expr::Symbol("cons".to_string()),
            Expr::Symbol("a".to_string()),
            Expr::Number(4),
        ]);
        let result2 = Evaluator::eval(&cons_expr2, &mut env).unwrap();
        assert_eq!(
            result2,
            Expr::DottedPair(
                Box::new(Expr::List(vec![Expr::Number(1), Expr::Number(2), Expr::Number(3)])),
                Box::new(Expr::Number(4))
            )
        );
    }
}
