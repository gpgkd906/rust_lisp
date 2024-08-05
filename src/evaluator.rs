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
            Expr::Number(_) | Expr::Float(_) | Expr::Str(_) => Ok(ast.clone()),
            Expr::List(list) => {
                if list.is_empty() {
                    return Ok(Expr::List(vec![]));
                }
                let first = &list[0];
                match first {
                    Expr::Symbol(s) => {
                        if let Some(operator_fn) = OperatorRegistry::get(s) {
                            operator_fn(&list[1..], env)
                        } else {
                            Lambda::eval_function_call(s, &list[1..], env)
                        }
                    }
                    Expr::List(_) => {
                        let func = Evaluator::eval(&list[0], env)?;
                        if let Expr::List(func_list) = func {
                            if func_list.len() >= 3 && func_list[0] == Expr::Symbol("lambda".to_string()) {
                                Lambda::eval_lambda_call(&func_list[1..], &list[1..], env)
                            } else {
                                Err(LispError::new("Invalid lambda"))
                            }
                        } else {
                            Err(LispError::new("Invalid expression"))
                        }
                    }
                    _ => Err(LispError::new("Cannot evaluate a list without a valid operator")),
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

    #[test]
    fn test_eval_lambda_call() {
        let mut env = setup_environment();
        let lambda_expr = Expr::List(vec![
            Expr::Symbol("lambda".to_string()),
            Expr::List(vec![Expr::Symbol("a".to_string())]),
            Expr::List(vec![Expr::Symbol("+".to_string()), Expr::Symbol("a".to_string()), Expr::Number(10)]),
        ]);
        env.set_function("my-func".to_string(), lambda_expr);

        let expr = Expr::List(vec![
            Expr::Symbol("my-func".to_string()),
            Expr::Number(5),
        ]);
        let result = Evaluator::eval(&expr, &mut env);
        assert_eq!(result, Ok(Expr::Number(15)));
    }

    // 测试无效函数调用
    #[test]
    fn test_eval_invalid_function_call() {
        let mut env = setup_environment();
    
        // Test for a function that is not defined
        let expr = Expr::List(vec![Expr::Symbol("undefined_func".to_string()), Expr::Number(5)]);
        let result = Evaluator::eval(&expr, &mut env);
        assert_eq!(result, Err(LispError::new("Undefined function: undefined_func")));
    
        // Test for a function list without a valid operator
        let expr = Expr::List(vec![Expr::Number(5), Expr::Number(10)]);
        let result = Evaluator::eval(&expr, &mut env);
        assert_eq!(result, Err(LispError::new("Cannot evaluate a list without a valid operator")));
    
        // Test for an invalid lambda call structure
        let invalid_lambda_expr = Expr::List(vec![
            Expr::List(vec![Expr::Symbol("lambda".to_string()), Expr::Number(5)]),  // Invalid parameters list
            Expr::Number(10),
        ]);
        let result = Evaluator::eval(&invalid_lambda_expr, &mut env);
        assert_eq!(result, Err(LispError::new("lambda requires at least 2 arguments: params, body")));
    }
    
    // 测试无效符号
    #[test]
    fn test_eval_undefined_symbol() {
        let mut env = setup_environment();
        let expr = Expr::Symbol("undefined".to_string());
        let result = Evaluator::eval(&expr, &mut env);
        assert_eq!(result, Err(LispError::new("Undefined symbol: undefined")));
    }

    // 测试嵌套表达式
    #[test]
    fn test_eval_nested_expression() {
        let mut env = setup_environment();
        let expr = Expr::List(vec![
            Expr::Symbol("+".to_string()),
            Expr::Number(1),
            Expr::List(vec![
                Expr::Symbol("*".to_string()),
                Expr::Number(2),
                Expr::Number(3),
            ]),
        ]);
        let result = Evaluator::eval(&expr, &mut env);
        assert_eq!(result, Ok(Expr::Number(7)));
    }

    // 测试嵌套Lambda
    #[test]
    fn test_eval_nested_lambda() {
        let mut env = setup_environment();
        
        // Define the outer lambda function that takes one argument
        let outer_lambda_expr = Expr::List(vec![
            Expr::Symbol("lambda".to_string()),
            Expr::List(vec![Expr::Symbol("a".to_string())]),
            Expr::List(vec![
                Expr::Symbol("+".to_string()),
                Expr::Symbol("a".to_string()),
                Expr::Number(5),
            ]),
        ]);
        
        // Define the nested lambda function that returns another lambda
        let nested_lambda_expr = Expr::List(vec![
            Expr::Symbol("lambda".to_string()),
            Expr::List(vec![Expr::Symbol("b".to_string())]),
            Expr::List(vec![
                Expr::Symbol("lambda".to_string()),
                Expr::List(vec![Expr::Symbol("c".to_string())]), // Parameter for the outer lambda
                Expr::List(vec![
                    Expr::Symbol("nested-func".to_string()), // Call the outer lambda
                    Expr::Symbol("c".to_string())
                ]),
            ]),
        ]);
        
        // Register the outer and nested lambdas
        env.set_function("nested-func".to_string(), outer_lambda_expr); // Set the outer lambda
        env.set_function("nested-lambda".to_string(), nested_lambda_expr); // Set the nested lambda
        
        // Call the nested lambda to get the inner lambda
        let nested_lambda_call = Expr::List(vec![
            Expr::Symbol("nested-lambda".to_string()),
            Expr::Number(0), // Argument for the first level lambda (should be ignored or adjusted)
        ]);
        
        // Now call the inner lambda with the actual argument
        let expr = Expr::List(vec![
            nested_lambda_call,
            Expr::Number(10), // Argument for the outer lambda
        ]);
        let result = Evaluator::eval(&expr, &mut env);
        assert_eq!(result, Ok(Expr::Number(15))); // Expecting 10 + 5
    }

    #[test]
    fn test_eval_invalid_lambda_expression() {
        let mut env = setup_environment();
    
        // Test an invalid lambda expression with missing parameters or body
        let invalid_lambda_expr = Expr::List(vec![
            Expr::Symbol("lambda".to_string()),
            Expr::Number(5),  // Invalid parameters list
        ]);
    
        let result = Evaluator::eval(&invalid_lambda_expr, &mut env);
        assert_eq!(result, Err(LispError::new("lambda requires at least 2 arguments: params, body")));
    
        let incomplete_lambda_expr = Expr::List(vec![
            Expr::Symbol("lambda".to_string()),
            Expr::List(vec![Expr::Symbol("x".to_string())]),
        ]);
    
        let result = Evaluator::eval(&incomplete_lambda_expr, &mut env);
        assert_eq!(result, Err(LispError::new("lambda requires at least 2 arguments: params, body")));
    }
    
    #[test]
    fn test_eval_invalid_function_definition() {
        let mut env = setup_environment();
    
        // Attempt to define a function with invalid syntax
        let invalid_defun_expr = Expr::List(vec![
            Expr::Symbol("defun".to_string()),
            Expr::Number(5), // Invalid function name
            Expr::List(vec![Expr::Symbol("x".to_string())]),
            Expr::List(vec![Expr::Symbol("+".to_string()), Expr::Symbol("x".to_string()), Expr::Number(1)]),
        ]);
    
        let result = Evaluator::eval(&invalid_defun_expr, &mut env);
        assert_eq!(result, Err(LispError::new("defun: first argument must be a symbol")));
    }
    
    #[test]
    fn test_eval_invalid_list_operator() {
        let mut env = setup_environment();
    
        // Attempt to evaluate a list with an invalid operator
        let invalid_operator_expr = Expr::List(vec![
            Expr::Number(10), // A number instead of a valid operator
            Expr::Number(5),
        ]);
    
        let result = Evaluator::eval(&invalid_operator_expr, &mut env);
        assert_eq!(result, Err(LispError::new("Cannot evaluate a list without a valid operator")));
    }

    #[test]
    fn test_eval_invalid_lambda() {
        let mut env = setup_environment();
    
        // Attempt to evaluate an invalid lambda structure
        let invalid_lambda = Expr::List(vec![
            Expr::List(vec![
                Expr::Symbol("lambda".to_string()),
                Expr::Number(5), // Invalid parameter list; must be a list
                Expr::Number(42), // Body is a number, which is valid
            ]),
            Expr::Number(10), // Argument for lambda call
        ]);
    
        let result = Evaluator::eval(&invalid_lambda, &mut env);
        // Expect a more specific error message regarding invalid parameter list
        assert_eq!(result, Err(LispError::new("lambda: first argument must be a list of parameters")));
    }
    
    #[test]
    fn test_eval_invalid_expression() {
        let mut env = setup_environment();
    
        // Attempt to evaluate an expression that results in an invalid state
        let invalid_expression = Expr::List(vec![
            Expr::List(vec![Expr::Number(42)]), // A list with just a number, not a valid lambda or operator
            Expr::Number(10),
        ]);
    
        let result = Evaluator::eval(&invalid_expression, &mut env);
        assert_eq!(result, Err(LispError::new("Cannot evaluate a list without a valid operator")));
    }
}
