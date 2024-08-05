// evaluator.rs
use crate::operator::OperatorRegistry;
use crate::environment::Environment;
use crate::exception::LispError;
use crate::expression::Expr;
use crate::operator::lambda::Lambda;

pub struct Evaluator;

impl Evaluator {
    pub fn eval(ast: &Expr, env: &mut Environment) -> Result<Expr, LispError> {
        match ast {
            Expr::Symbol(symbol) => {
                env.get_symbol(symbol)
                    .cloned()
                    .ok_or_else(|| LispError::new(&format!("Undefined symbol: {}", symbol)))
            }
            Expr::Number(_) | Expr::Str(_) => Ok(ast.clone()),
            Expr::List(list) => {
                if list.is_empty() {
                    return Ok(Expr::List(vec![]));
                }
                let first = &list[0];
                if let Expr::Symbol(s) = first {
                    if let Some(operator_fn) = OperatorRegistry::get(s) {
                        operator_fn(&list[1..], env)
                    } else {
                        Lambda::eval_function_call(s, &list[1..], env)
                    }
                } else {
                    Err(LispError::new("Cannot evaluate a list without a valid operator"))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::environment::Environment;
    use crate::expression::Expr;

    fn setup_environment() -> Environment {
        let mut env = Environment::initialize(); // Use initialize instead of new
        // Initialize the environment with some predefined symbols if needed
        env.set_symbol("x".to_string(), Expr::Number(10));
        env.set_symbol("y".to_string(), Expr::Number(20));
        env
    }

    #[test]
    fn test_eval_symbol() {
        let mut env = setup_environment();
        let expr = Expr::Symbol("x".to_string());
        let result = Evaluator::eval(&expr, &mut env);
        assert_eq!(result, Ok(Expr::Number(10)));

        let expr = Expr::Symbol("z".to_string());
        let result = Evaluator::eval(&expr, &mut env);
        assert!(result.is_err());
    }

    #[test]
    fn test_eval_number_and_string() {
        let mut env = setup_environment();
        let expr = Expr::Number(42);
        let result = Evaluator::eval(&expr, &mut env);
        assert_eq!(result, Ok(Expr::Number(42)));

        let expr = Expr::Str("Hello".to_string());
        let result = Evaluator::eval(&expr, &mut env);
        assert_eq!(result, Ok(Expr::Str("Hello".to_string())));
    }

    #[test]
    fn test_eval_empty_list() {
        let mut env = setup_environment();
    
        let result = Evaluator::eval(&Expr::List(vec![]), &mut env);
    
        // 修改断言为期待的结果
        assert_eq!(result, Ok(Expr::List(vec![]))); // 符合实现
    }
}
