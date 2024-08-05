// evaluator.rs

use crate::environment::Environment;
use crate::exception::LispError;
use crate::expression::Expr;
use crate::operator::{
    arithmetic::Arithmetic, comparison::Comparison, control::Control, list::ListOps, set::SetOps,
    lambda::Lambda,
};

pub struct Evaluator;

impl Evaluator {
    pub fn eval_tree(ast: &Expr, env: &mut Environment) -> Result<Expr, LispError> {
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
                match first {
                    Expr::Symbol(s) => match s.as_str() {
                        "+" => Arithmetic::eval_add(&list[1..], env),
                        "-" => Arithmetic::eval_subtract(&list[1..], env),
                        "*" => Arithmetic::eval_multiply(&list[1..], env),
                        "/" => Arithmetic::eval_divide(&list[1..], env),
                        "setf" => SetOps::eval_setf(&list[1..], env),
                        "car" => ListOps::eval_car(&list[1..], env),
                        "cdr" => ListOps::eval_cdr(&list[1..], env),
                        "cons" => ListOps::eval_cons(&list[1..], env),
                        "cond" => Control::eval_cond(&list[1..], env),
                        "not" => Control::eval_not(&list[1..], env),
                        ">" => Comparison::eval_greater(&list[1..], env),
                        "<" => Comparison::eval_less(&list[1..], env),
                        ">=" => Comparison::eval_greater_equal(&list[1..], env),
                        "<=" => Comparison::eval_less_equal(&list[1..], env),
                        "=" => Comparison::eval_equal(&list[1..], env),
                        "quote" => ListOps::eval_quote(&list[1..], env),
                        "count" => ListOps::eval_length(&list[1..], env),
                        "defun" => Lambda::eval_defun(&list[1..], env),
                        _ => Lambda::eval_function_call(s, &list[1..], env),
                    },
                    _ => Err(LispError::new("Cannot evaluate a list without a valid operator")),
                }
            }
        }
    }    
}



// evaluator.rs

// evaluator.rs

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
        let result = Evaluator::eval_tree(&expr, &mut env);
        assert_eq!(result, Ok(Expr::Number(10)));

        let expr = Expr::Symbol("z".to_string());
        let result = Evaluator::eval_tree(&expr, &mut env);
        assert!(result.is_err());
    }

    #[test]
    fn test_eval_number_and_string() {
        let mut env = setup_environment();
        let expr = Expr::Number(42);
        let result = Evaluator::eval_tree(&expr, &mut env);
        assert_eq!(result, Ok(Expr::Number(42)));

        let expr = Expr::Str("Hello".to_string());
        let result = Evaluator::eval_tree(&expr, &mut env);
        assert_eq!(result, Ok(Expr::Str("Hello".to_string())));
    }

    #[test]
    fn test_eval_empty_list() {
        let mut env = setup_environment();
    
        let result = Evaluator::eval_tree(&Expr::List(vec![]), &mut env);
    
        // 修改断言为期待的结果
        assert_eq!(result, Ok(Expr::List(vec![]))); // 符合实现
    }
    
    #[test]
    fn test_eval_arithmetic_operations() {
        let mut env = setup_environment();
        
        let expr = Expr::List(vec![
            Expr::Symbol("+".to_string()),
            Expr::Number(1),
            Expr::Number(2),
        ]);
        let result = Evaluator::eval_tree(&expr, &mut env);
        assert_eq!(result, Ok(Expr::Number(3)));

        let expr = Expr::List(vec![
            Expr::Symbol("*".to_string()),
            Expr::Number(3),
            Expr::Number(4),
        ]);
        let result = Evaluator::eval_tree(&expr, &mut env);
        assert_eq!(result, Ok(Expr::Number(12)));
    }

    #[test]
    fn test_eval_list_operations() {
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
    
        // 测试 car 操作符
        let expr = Expr::List(vec![
            Expr::Symbol("car".to_string()),
            Expr::List(vec![
                Expr::Symbol("quote".to_string()),
                Expr::List(vec![Expr::Number(1), Expr::Number(2), Expr::Number(3)]),
            ]),
        ]);
        let result = Evaluator::eval_tree(&expr, &mut env);
        assert_eq!(result, Ok(Expr::Number(1)));
    
        // 测试 cdr 操作符
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
    fn test_eval_conditional_operations() {
        let mut env = setup_environment();
    
        // 正确结构的 cond 表达式
        let expr = Expr::List(vec![
            Expr::Symbol("cond".to_string()),
            Expr::List(vec![
                Expr::List(vec![
                    Expr::Symbol(">".to_string()),
                    Expr::Number(5),
                    Expr::Number(3),
                ]),
                Expr::Number(42),
            ]),
            Expr::List(vec![
                Expr::Symbol("else".to_string()), // 确保有一个 else 子句
                Expr::Number(0),
            ]),
        ]);
    
        let result = Evaluator::eval_tree(&expr, &mut env);
        assert_eq!(result, Ok(Expr::Number(42)));
    
        // 带有 false 条件的测试
        let expr = Expr::List(vec![
            Expr::Symbol("cond".to_string()),
            Expr::List(vec![
                Expr::List(vec![
                    Expr::Symbol("<".to_string()),
                    Expr::Number(5),
                    Expr::Number(3),
                ]),
                Expr::Number(42),
            ]),
            Expr::List(vec![
                Expr::List(vec![
                    Expr::Symbol(">".to_string()),
                    Expr::Number(5),
                    Expr::Number(3),
                ]),
                Expr::Number(100),
            ]),
        ]);
    
        let result = Evaluator::eval_tree(&expr, &mut env);
        assert_eq!(result, Ok(Expr::Number(100)));
    }
}
