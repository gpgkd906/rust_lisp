// operator/lambda.rs
use crate::operator::OperatorRegistry;
use crate::environment::Environment;
use crate::exception::LispError;
use crate::expression::Expr;
use crate::Evaluator;

pub struct Lambda;

impl Lambda {
    pub fn eval_defun(args: &[Expr], env: &mut Environment) -> Result<Expr, LispError> {
        if args.len() != 3 {
            return Err(LispError::new("defun requires exactly 3 arguments: name, params, body"));
        }
    
        let func_name = match &args[0] {
            Expr::Symbol(s) => s.clone(),
            _ => return Err(LispError::new("defun: first argument must be a symbol")),
        };
    
        let params = match &args[1] {
            Expr::List(list) => list.clone(),
            _ => return Err(LispError::new("defun: second argument must be a list")),
        };
    
        let body = args[2].clone();
    
        // 使用 set_function 将函数存储在函数符号表中
        env.set_function(
            func_name.clone(),
            Expr::List(vec![Expr::Symbol("lambda".to_string()), Expr::List(params), body]),
        );
    
        Ok(Expr::Symbol(func_name))
    }
    
    pub fn eval_function_call(func_name: &str, args: &[Expr], env: &mut Environment) -> Result<Expr, LispError> {
        let function = env
            .get_function(func_name)
            .ok_or_else(|| LispError::new(&format!("Undefined function: {}", func_name)))?;
    
        if let Expr::List(list) = function {
            if list.len() != 3 || list[0] != Expr::Symbol("lambda".to_string()) {
                return Err(LispError::new("Invalid function definition"));
            }
    
            let params = if let Expr::List(p) = &list[1] {
                p
            } else {
                return Err(LispError::new("Invalid parameter list"));
            };
    
            if params.len() != args.len() {
                return Err(LispError::new("Argument count does not match parameter count"));
            }
    
            let mut local_env = env.clone();
            for (param, arg) in params.iter().zip(args.iter()) {
                if let Expr::Symbol(s) = param {
                    let value = Evaluator::eval(arg, &mut local_env)?;
                    local_env.set_symbol(s.clone(), value);
                } else {
                    return Err(LispError::new("Invalid parameter name"));
                }
            }
    
            Evaluator::eval(&list[2], &mut local_env)
        } else {
            Err(LispError::new("Function is not defined correctly"))
        }
    }    
}

pub fn register_lambda_operators() {
    OperatorRegistry::register("defun", Lambda::eval_defun);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::environment::Environment;
    use crate::expression::Expr;

    fn setup_environment() -> Environment {
        let mut env = Environment::initialize();
        env.set_symbol("nil".to_string(), Expr::List(vec![]));
        env.set_symbol("t".to_string(), Expr::Symbol("t".to_string()));
        env
    }

    #[test]
    fn test_eval_defun_success() {
        let mut env = setup_environment();
        
        // 定义一个简单的加法函数
        let defun_expr = Expr::List(vec![
            Expr::Symbol("add".to_string()),   // 函数名
            Expr::List(vec![                   // 参数列表
                Expr::Symbol("a".to_string()), 
                Expr::Symbol("b".to_string())
            ]),
            Expr::List(vec![                   // 函数体
                Expr::Symbol("+".to_string()),
                Expr::Symbol("a".to_string()),
                Expr::Symbol("b".to_string()),
            ]),
        ]);
    
        if let Expr::List(ref list) = defun_expr {
            let result = Lambda::eval_defun(list, &mut env);
            assert_eq!(result, Ok(Expr::Symbol("add".to_string())));
    
            // 验证函数是否正确存储在函数表中
            let function = env.get_function("add");
            assert!(function.is_some());
            if let Some(Expr::List(list)) = function {
                assert_eq!(list[0], Expr::Symbol("lambda".to_string()));
            }
        } else {
            panic!("Defun expression is not a list");
        }
    }    
    
    #[test]
    fn test_eval_function_call_success() {
        let mut env = setup_environment();
    
        // 预定义一个加法函数
        env.set_function(
            "add".to_string(),
            Expr::List(vec![
                Expr::Symbol("lambda".to_string()),
                Expr::List(vec![Expr::Symbol("a".to_string()), Expr::Symbol("b".to_string())]),
                Expr::List(vec![
                    Expr::Symbol("+".to_string()),
                    Expr::Symbol("a".to_string()),
                    Expr::Symbol("b".to_string()),
                ]),
            ]),
        );
    
        // 调用 add 函数
        let call_expr = Expr::List(vec![
            Expr::Symbol("add".to_string()),
            Expr::Number(3),
            Expr::Number(4),
        ]);
    
        if let Expr::List(ref list) = call_expr {
            let result = Lambda::eval_function_call("add", &list[1..], &mut env);
            assert_eq!(result, Ok(Expr::Number(7)));
        } else {
            panic!("Function call expression is not a list");
        }
    }    

    #[test]
    fn test_eval_defun_invalid_arguments() {
        let mut env = setup_environment();

        // 定义函数时使用错误参数数量
        let defun_expr = Expr::List(vec![
            Expr::Symbol("defun".to_string()),
            Expr::Symbol("add".to_string()),
            Expr::Symbol("a".to_string()), // 错误：参数应为列表
            Expr::List(vec![
                Expr::Symbol("+".to_string()),
                Expr::Symbol("a".to_string()),
                Expr::Symbol("b".to_string()),
            ]),
        ]);

        if let Expr::List(ref list) = defun_expr {
            let result = Lambda::eval_defun(list, &mut env);
            assert!(result.is_err());
        } else {
            panic!("Defun expression is not a list");
        }

        // 定义函数时使用非符号作为函数名
        let defun_expr = Expr::List(vec![
            Expr::Symbol("defun".to_string()),
            Expr::Number(123), // 错误：函数名应为符号
            Expr::List(vec![Expr::Symbol("a".to_string()), Expr::Symbol("b".to_string())]),
            Expr::List(vec![
                Expr::Symbol("+".to_string()),
                Expr::Symbol("a".to_string()),
                Expr::Symbol("b".to_string()),
            ]),
        ]);

        if let Expr::List(ref list) = defun_expr {
            let result = Lambda::eval_defun(list, &mut env);
            assert!(result.is_err());
        } else {
            panic!("Defun expression is not a list");
        }
    }

    #[test]
    fn test_eval_function_call_invalid_arguments() {
        let mut env = setup_environment();

        // 预定义一个加法函数
        env.set_function(
            "add".to_string(),
            Expr::List(vec![
                Expr::Symbol("lambda".to_string()),
                Expr::List(vec![Expr::Symbol("a".to_string()), Expr::Symbol("b".to_string())]),
                Expr::List(vec![
                    Expr::Symbol("+".to_string()),
                    Expr::Symbol("a".to_string()),
                    Expr::Symbol("b".to_string()),
                ]),
            ]),
        );

        // 调用 add 函数时参数数量不匹配
        let call_expr = Expr::List(vec![
            Expr::Symbol("add".to_string()),
            Expr::Number(3),
        ]);

        if let Expr::List(ref list) = call_expr {
            let result = Lambda::eval_function_call("add", &list[1..], &mut env);
            assert!(result.is_err());
        } else {
            panic!("Function call expression is not a list");
        }
    }

    #[test]
    fn test_eval_function_call_undefined_function() {
        let mut env = setup_environment();

        // 调用未定义的函数
        let call_expr = Expr::List(vec![
            Expr::Symbol("undefined_func".to_string()),
            Expr::Number(3),
            Expr::Number(4),
        ]);

        if let Expr::List(ref list) = call_expr {
            let result = Lambda::eval_function_call("undefined_func", &list[1..], &mut env);
            assert!(result.is_err());
        } else {
            panic!("Function call expression is not a list");
        }
    }

    #[test]
    fn test_eval_fib_function() {
        let mut env = setup_environment();
    
        // 定义 Fibonacci 函数
        let fib_defun = Expr::List(vec![
            Expr::Symbol("fib".to_string()), // 函数名
            Expr::List(vec![Expr::Symbol("n".to_string())]), // 参数列表
            Expr::List(vec![ // 函数体
                Expr::Symbol("cond".to_string()), 
                Expr::List(vec![
                    Expr::List(vec![Expr::Symbol("eq".to_string()), Expr::Symbol("n".to_string()), Expr::Number(1)]),
                    Expr::Number(1),
                ]),
                Expr::List(vec![
                    Expr::List(vec![Expr::Symbol("eq".to_string()), Expr::Symbol("n".to_string()), Expr::Number(0)]),
                    Expr::Number(0),
                ]),
                Expr::List(vec![
                    Expr::Symbol("t".to_string()),
                    Expr::List(vec![
                        Expr::Symbol("+".to_string()),
                        Expr::List(vec![
                            Expr::Symbol("fib".to_string()),
                            Expr::List(vec![Expr::Symbol("-".to_string()), Expr::Symbol("n".to_string()), Expr::Number(1)]),
                        ]),
                        Expr::List(vec![
                            Expr::Symbol("fib".to_string()),
                            Expr::List(vec![Expr::Symbol("-".to_string()), Expr::Symbol("n".to_string()), Expr::Number(2)]),
                        ]),
                    ]),
                ]),
            ]),
        ]);

        if let Expr::List(ref list) = fib_defun {
            let result = Lambda::eval_defun(list, &mut env);
            assert_eq!(result, Ok(Expr::Symbol("fib".to_string())));
        } else {
            panic!("Fib defun expression is not a list");
        }

        // 调用 Fibonacci 函数
        let fib_call = Expr::List(vec![
            Expr::Symbol("fib".to_string()),
            Expr::Number(6),
        ]);

        if let Expr::List(ref list) = fib_call {
            let result = Lambda::eval_function_call("fib", &list[1..], &mut env);
            assert_eq!(result, Ok(Expr::Number(8)));
        } else {
            panic!("Fib call expression is not a list");
        }
    }
}
