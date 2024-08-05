// operator/control.rs
use crate::operator::OperatorRegistry;
use crate::environment::Environment;
use crate::exception::LispError;
use crate::expression::Expr;
use crate::evaluator::Evaluator;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use lazy_static::lazy_static;

pub struct Control;

// 全局原子计数器
lazy_static! {
    static ref GENSYM_COUNTER: Mutex<AtomicUsize> = Mutex::new(AtomicUsize::new(0));
}

impl Control {
    pub fn eval_cond(conditions: &[Expr], env: &mut Environment) -> Result<Expr, LispError> {
        for condition in conditions {
            match condition {
                Expr::List(pair) => {
                    if pair.len() == 1 {
                        // 如果子句只有一个元素，直接返回该元素的值
                        return Evaluator::eval(&pair[0], env);
                    } else if pair.len() == 2 {
                        let test = &pair[0];
                        let result = &pair[1];

                        let test_value = match Evaluator::eval(test, env) {
                            Ok(Expr::Symbol(s)) if s == "t" || s == "T" => true,  // 支持真值符号 t 或 T
                            Ok(Expr::Number(n)) if n != 0 => true,   // 非零数值作为真值
                            Ok(Expr::List(list)) if !list.is_empty() => true, // 非空列表为真
                            Ok(Expr::Symbol(s)) if s == "nil" => false, // nil 为假值
                            Ok(_) => false,
                            Err(_) => false,
                        };

                        if test_value {
                            return Evaluator::eval(result, env);
                        }
                    } else {
                        return Err(LispError::new("Each cond clause must have exactly one or two elements"));
                    }
                }
                _ => return Err(LispError::new("Cond clause must be a list")),
            }
        }
        Err(LispError::new("No true condition in cond"))
    }


    pub fn eval_not(args: &[Expr], _env: &mut Environment) -> Result<Expr, LispError> {
        if args.len() != 1 {
            return Err(LispError::new("not expects exactly one argument"));
        }

        // 直接识别假值，不调用 eval_tree
        let is_false = match &args[0] {
            Expr::Symbol(ref s) if s == "nil" => true,  // nil 为假
            Expr::Number(n) if *n == 0 => true,          // 0 为假
            Expr::List(ref list) if list.is_empty() => true, // 空列表为假
            _ => false,
        };

        if is_false {
            Ok(Expr::Symbol("t".to_string())) // 返回真值 t
        } else {
            Ok(Expr::List(vec![])) // 返回假值 nil
        }
    }

    pub fn eval_gensym(_: &[Expr], _: &mut Environment) -> Result<Expr, LispError> {
        let mut counter = GENSYM_COUNTER.lock().unwrap();
        let gensym_id = counter.fetch_add(1, Ordering::SeqCst);
        Ok(Expr::Symbol(format!("#:G{}", gensym_id)))
    }
}

pub fn register_control_operators() {
    OperatorRegistry::register("cond", Control::eval_cond);
    OperatorRegistry::register("not", Control::eval_not);
    OperatorRegistry::register("gensym", Control::eval_gensym);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::environment::Environment;
    use crate::expression::Expr;
    use crate::evaluator::Evaluator;

    fn setup_environment() -> Environment {
        let mut env = Environment::initialize();
        env.set_symbol("nil".to_string(), Expr::List(vec![])); // 将 nil 设为空列表
        env.set_symbol("t".to_string(), Expr::Symbol("t".to_string())); // 真值 t
        env.set_symbol("T".to_string(), Expr::Symbol("T".to_string())); // 大写 T 作为真值
        env
    }

    #[test]
    fn test_cond_single_expression() {
        let mut env = setup_environment();

        // Test case: (cond (3)) should be 3
        let expr = Expr::List(vec![
            Expr::Symbol("cond".to_string()),
            Expr::List(vec![Expr::Number(3)]),
        ]);

        let result = Evaluator::eval(&expr, &mut env);
        assert_eq!(result, Ok(Expr::Number(3)));
    }

    #[test]
    fn test_cond_with_multiple_clauses() {
        let mut env = setup_environment();

        // Test case: (cond (nil 1) ((not nil) 2) (3)) should be 2
        let expr = Expr::List(vec![
            Expr::Symbol("cond".to_string()),
            Expr::List(vec![
                Expr::List(vec![]),
                Expr::Number(1),
            ]),
            Expr::List(vec![
                Expr::List(vec![
                    Expr::Symbol("not".to_string()),
                    Expr::List(vec![]),
                ]),
                Expr::Number(2),
            ]),
            Expr::List(vec![Expr::Number(3)]),
        ]);

        let result = Evaluator::eval(&expr, &mut env);
        assert_eq!(result, Ok(Expr::Number(2)));
    }

    #[test]
    fn test_cond_with_symbol_true() {
        let mut env = setup_environment();

        // Test case: (cond ((not t) 1) (t 2)) should be 2
        let expr = Expr::List(vec![
            Expr::Symbol("cond".to_string()),
            Expr::List(vec![
                Expr::List(vec![
                    Expr::Symbol("not".to_string()),
                    Expr::Symbol("t".to_string()),
                ]),
                Expr::Number(1),
            ]),
            Expr::List(vec![
                Expr::Symbol("t".to_string()),
                Expr::Number(2),
            ]),
        ]);

        let result = Evaluator::eval(&expr, &mut env);
        assert_eq!(result, Ok(Expr::Number(2)));
    }

    #[test]
    fn test_cond_with_nonempty_list_true() {
        let mut env = setup_environment();
    
        // Test case: (cond (() 1) ((list 1) 2) (t 3)) should be 2
        let expr = Expr::List(vec![
            Expr::Symbol("cond".to_string()),
            Expr::List(vec![
                Expr::List(vec![]), // 空列表，为假
                Expr::Number(1),
            ]),
            Expr::List(vec![
                Expr::List(vec![
                    Expr::Symbol("quote".to_string()), // 使用 quote 确保列表不被求值
                    Expr::List(vec![Expr::Number(1)]), // 非空列表，为真
                ]),
                Expr::Number(2),
            ]),
            Expr::List(vec![
                Expr::Symbol("t".to_string()),
                Expr::Number(3),
            ]),
        ]);
    
        let result = Evaluator::eval(&expr, &mut env);
        assert_eq!(result, Ok(Expr::Number(2)));
    }
    
    #[test]
    fn test_cond_with_multiple_true_conditions() {
        let mut env = setup_environment();

        // Test case: (cond (t 1) (t 2) (3)) should be 1
        let expr = Expr::List(vec![
            Expr::Symbol("cond".to_string()),
            Expr::List(vec![
                Expr::Symbol("t".to_string()),
                Expr::Number(1),
            ]),
            Expr::List(vec![
                Expr::Symbol("t".to_string()),
                Expr::Number(2),
            ]),
            Expr::List(vec![Expr::Number(3)]),
        ]);

        let result = Evaluator::eval(&expr, &mut env);
        assert_eq!(result, Ok(Expr::Number(1)));
    }

    #[test]
    fn test_not_operator() {
        let mut env = setup_environment();

        // Test not operator on various inputs
        let result = Control::eval_not(&[Expr::List(vec![])], &mut env);
        assert_eq!(result, Ok(Expr::Symbol("t".to_string())));

        let result = Control::eval_not(&[Expr::Number(1)], &mut env);
        assert_eq!(result, Ok(Expr::List(vec![])));

        let result = Control::eval_not(&[Expr::Number(0)], &mut env);
        assert_eq!(result, Ok(Expr::Symbol("t".to_string())));

        let result = Control::eval_not(&[Expr::List(vec![])], &mut env);
        assert_eq!(result, Ok(Expr::Symbol("t".to_string())));

        let result = Control::eval_not(&[Expr::List(vec![Expr::Number(1)])], &mut env);
        assert_eq!(result, Ok(Expr::List(vec![])));

        let result = Control::eval_not(&[Expr::Symbol("t".to_string())], &mut env);
        assert_eq!(result, Ok(Expr::List(vec![])));

        let result = Control::eval_not(&[Expr::Symbol("T".to_string())], &mut env);
        assert_eq!(result, Ok(Expr::List(vec![])));

        // Testing nested lists with not
        let result = Control::eval_not(&[
            Expr::List(vec![
                Expr::List(vec![Expr::Number(1)]),
                Expr::Number(2)
            ])
        ], &mut env);
        assert_eq!(result, Ok(Expr::List(vec![])));
    }

    #[test]
    fn test_cond_single_clause() {
        let mut env = setup_environment();
    
        // 测试：(cond (5)) 应返回 5
        let expr = Expr::List(vec![
            Expr::Symbol("cond".to_string()),
            Expr::List(vec![Expr::Number(5)]),
        ]);
    
        let result = Evaluator::eval(&expr, &mut env);
        assert_eq!(result, Ok(Expr::Number(5)));
    }

    #[test]
    fn test_cond_no_true_condition() {
        let mut env = setup_environment();
    
        // 测试：(cond (nil 1) (nil 2)) 应返回错误
        let expr = Expr::List(vec![
            Expr::Symbol("cond".to_string()),
            Expr::List(vec![
                Expr::Symbol("nil".to_string()),
                Expr::Number(1),
            ]),
            Expr::List(vec![
                Expr::Symbol("nil".to_string()),
                Expr::Number(2),
            ]),
        ]);
    
        let result = Evaluator::eval(&expr, &mut env);
        assert_eq!(result, Err(LispError::new("No true condition in cond")));
    }

    #[test]
    fn test_cond_clause_too_long() {
        let mut env = setup_environment();
    
        // 测试：(cond (t 1 2)) 应抛出错误
        let expr = Expr::List(vec![
            Expr::Symbol("cond".to_string()),
            Expr::List(vec![
                Expr::Symbol("t".to_string()),
                Expr::Number(1),
                Expr::Number(2),
            ]),
        ]);
    
        let result = Evaluator::eval(&expr, &mut env);
        assert_eq!(result, Err(LispError::new("Each cond clause must have exactly one or two elements")));
    }

    #[test]
    fn test_cond_clause_not_a_list() {
        let mut env = setup_environment();
    
        // 测试：(cond 1) 应抛出错误
        let expr = Expr::List(vec![
            Expr::Symbol("cond".to_string()),
            Expr::Number(1),
        ]);
    
        let result = Evaluator::eval(&expr, &mut env);
        assert_eq!(result, Err(LispError::new("Cond clause must be a list")));
    }

    #[test]
    fn test_gensym_unique_symbols() {
        let mut env = setup_environment();
        
        let gensym1 = Control::eval_gensym(&[], &mut env).unwrap();
        let gensym2 = Control::eval_gensym(&[], &mut env).unwrap();

        assert_ne!(gensym1, gensym2);
        
        if let (Expr::Symbol(sym1), Expr::Symbol(sym2)) = (gensym1, gensym2) {
            assert!(sym1.starts_with("#:G"));
            assert!(sym2.starts_with("#:G"));
        } else {
            panic!("Gensym did not return symbols");
        }
    }

    #[test]
    fn test_setf_and_gensym() {
        let mut env = setup_environment();
        
        // [17] (setf seed (gensym))
        let setf_expr = Expr::List(vec![
            Expr::Symbol("setf".to_string()),
            Expr::Symbol("seed".to_string()),
            Expr::List(vec![Expr::Symbol("gensym".to_string())]),
        ]);
        let result = Evaluator::eval(&setf_expr, &mut env).unwrap();
        
        if let Expr::Symbol(ref seed) = result { // 使用 ref 关键字避免移动
            assert!(seed.starts_with("#:G"));
        } else {
            panic!("Expected gensym result to be a symbol");
        }
    
        // [18] seed
        let seed_expr = Expr::Symbol("seed".to_string());
        let seed_result = Evaluator::eval(&seed_expr, &mut env).unwrap();
        assert_eq!(seed_result, result); // 确保 result 和 seed_result 相同
    }    
}
