use crate::environment::Environment;
use crate::exception::LispError;
use crate::expression::Expr;
use std::collections::HashMap;

pub struct MacroExpander;

impl MacroExpander {
    pub fn parse_defmacro(list: &[Expr], env: &mut Environment) -> Result<Expr, LispError> {
        if list.len() < 4 {
            return Err(LispError::new("defmacro: 需要至少三个参数：宏名、参数列表和宏体"));
        }

        // 获取宏的名称
        let macro_name = if let Expr::Symbol(name) = &list[1] {
            name.clone()
        } else {
            return Err(LispError::new("defmacro: 第一个参数必须是一个符号"));
        };

        // 获取参数列表
        let params = if let Expr::List(params) = &list[2] {
            params.clone()
        } else {
            return Err(LispError::new("defmacro: 第二个参数必须是一个参数列表"));
        };

        // 获取宏体
        let body = list[3].clone();

        // 存储宏定义到环境中
        env.set_macro(macro_name.clone(), Expr::Macro(params, Box::new(body)));

        // 返回一个空列表，表示宏定义不在运行时存在
        Ok(Expr::List(vec![]))
    }

    pub fn expand_macro(ast: &Expr, env: &mut Environment) -> Result<Expr, LispError> {
        match ast {
            Expr::List(list) => {
                if let Some(Expr::Symbol(s)) = list.first() {
                    if let Some(mac) = env.get_macro(s) {
                        let mac_clone = mac.clone();
                        return MacroExpander::expand(&mac_clone, &list[1..], env);
                    }
                }
                let expanded_list: Result<Vec<Expr>, LispError> = list
                    .iter()
                    .map(|expr| MacroExpander::expand_macro(expr, env))
                    .collect();
                Ok(Expr::List(expanded_list?))
            }
            _ => Ok(ast.clone()),
        }
    }

    fn expand(mac: &Expr, args: &[Expr], env: &mut Environment) -> Result<Expr, LispError> {
        if let Expr::Macro(params, template) = mac {
            if params.len() != args.len() {
                return Err(LispError::new("参数数量不匹配"));
            }

            let mut substitutions = HashMap::new();
            for (param, arg) in params.iter().zip(args.iter()) {
                if let Expr::Symbol(name) = param {
                    substitutions.insert(name.clone(), arg.clone());
                }
            }

            // 使用替换后的模板进行宏展开
            let result = MacroExpander::substitute(template, &substitutions)?;

            // 继续递归展开，处理嵌套宏
            MacroExpander::expand_macro(&result, env)
        } else {
            Err(LispError::new("不是有效的宏"))
        }
    }

    fn substitute(template: &Expr, substitutions: &HashMap<String, Expr>) -> Result<Expr, LispError> {
        match template {
            Expr::Symbol(name) => {
                if let Some(value) = substitutions.get(name) {
                    return Ok(value.clone());
                }
                Ok(template.clone())
            }
            Expr::List(list) => {
                if let Some(Expr::Symbol(ref s)) = list.first() {
                    match s.as_str() {
                        "quasiquote" => {
                            // Handle quasiquote (`)
                            if list.len() != 2 {
                                return Err(LispError::new("quasiquote: 需要一个参数"));
                            }
                            return MacroExpander::expand_quasiquote(&list[1], substitutions);
                        }
                        _ => {
                            let mut new_list = Vec::new();
                            for expr in list {
                                new_list.push(MacroExpander::substitute(expr, substitutions)?);
                            }
                            Ok(Expr::List(new_list))
                        }
                    }
                } else {
                    let mut new_list = Vec::new();
                    for expr in list {
                        new_list.push(MacroExpander::substitute(expr, substitutions)?);
                    }
                    Ok(Expr::List(new_list))
                }
            }
            _ => Ok(template.clone()),
        }
    }

    fn expand_quasiquote(expr: &Expr, substitutions: &HashMap<String, Expr>) -> Result<Expr, LispError> {
        match expr {
            Expr::List(list) => {
                let mut expanded_list = Vec::new();
                for item in list {
                    if let Expr::List(inner_list) = item {
                        if let Some(Expr::Symbol(ref s)) = inner_list.first() {
                            if s == "unquote" {
                                if inner_list.len() != 2 {
                                    return Err(LispError::new("unquote: 需要一个参数"));
                                }
                                let to_unquote = &inner_list[1];
                                if let Expr::Symbol(name) = to_unquote {
                                    if let Some(value) = substitutions.get(name) {
                                        expanded_list.push(value.clone());
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                    expanded_list.push(MacroExpander::expand_quasiquote(item, substitutions)?);
                }
                Ok(Expr::List(expanded_list))
            }
            Expr::Symbol(name) => {
                if let Some(value) = substitutions.get(name) {
                    return Ok(value.clone());
                }
                Ok(expr.clone())
            }
            _ => Ok(expr.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::environment::Environment;
    use crate::expression::Expr;
    use crate::exception::LispError;

    #[test]
    fn test_expand_macro_simple() {
        let mut env = Environment::initialize();
        env.set_macro("my-macro".to_string(), Expr::Macro(vec![Expr::Symbol("x".to_string())], Box::new(Expr::List(vec![
            Expr::Symbol("+".to_string()),
            Expr::Symbol("x".to_string()),
            Expr::Number(10)
        ]))));

        let ast = Expr::List(vec![
            Expr::Symbol("my-macro".to_string()),
            Expr::Number(5),
        ]);

        let result = MacroExpander::expand_macro(&ast, &mut env);
        assert!(result.is_ok());

        if let Ok(expanded) = result {
            assert_eq!(
                expanded,
                Expr::List(vec![
                    Expr::Symbol("+".to_string()),
                    Expr::Number(5),
                    Expr::Number(10)
                ])
            );
        }
    }

    #[test]
    fn test_expand_macro_with_no_args() {
        let mut env = Environment::initialize();
        env.set_macro("simple-macro".to_string(), Expr::Macro(vec![], Box::new(Expr::List(vec![
            Expr::Symbol("quote".to_string()),
            Expr::List(vec![
                Expr::Symbol("hello".to_string()),
                Expr::Symbol("world".to_string())
            ])
        ]))));

        let ast = Expr::List(vec![
            Expr::Symbol("simple-macro".to_string())
        ]);

        let result = MacroExpander::expand_macro(&ast, &mut env);
        assert!(result.is_ok());

        if let Ok(expanded) = result {
            assert_eq!(
                expanded,
                Expr::List(vec![
                    Expr::Symbol("quote".to_string()),
                    Expr::List(vec![
                        Expr::Symbol("hello".to_string()),
                        Expr::Symbol("world".to_string())
                    ])
                ])
            );
        }
    }

    #[test]
    fn test_expand_macro_with_multiple_args() {
        let mut env = Environment::initialize();
        env.set_macro("sum-macro".to_string(), Expr::Macro(vec![Expr::Symbol("a".to_string()), Expr::Symbol("b".to_string())], Box::new(Expr::List(vec![
            Expr::Symbol("+".to_string()),
            Expr::Symbol("a".to_string()),
            Expr::Symbol("b".to_string())
        ]))));

        let ast = Expr::List(vec![
            Expr::Symbol("sum-macro".to_string()),
            Expr::Number(3),
            Expr::Number(4),
        ]);

        let result = MacroExpander::expand_macro(&ast, &mut env);
        assert!(result.is_ok());

        if let Ok(expanded) = result {
            assert_eq!(
                expanded,
                Expr::List(vec![
                    Expr::Symbol("+".to_string()),
                    Expr::Number(3),
                    Expr::Number(4)
                ])
            );
        }
    }

    #[test]
    fn test_expand_macro_with_unexpected_args() {
        let mut env = Environment::initialize();
        env.set_macro("one-arg-macro".to_string(), Expr::Macro(vec![Expr::Symbol("x".to_string())], Box::new(Expr::Symbol("x".to_string()))));

        let ast = Expr::List(vec![
            Expr::Symbol("one-arg-macro".to_string()),
            Expr::Number(1),
            Expr::Number(2), // extra argument
        ]);

        let result = MacroExpander::expand_macro(&ast, &mut env);
        assert!(result.is_err());
    }

    #[test]
    fn test_expand_non_macro() {
        let mut env = Environment::initialize();
        let ast = Expr::List(vec![
            Expr::Symbol("+".to_string()),
            Expr::Number(1),
            Expr::Number(2),
        ]);

        let result = MacroExpander::expand_macro(&ast, &mut env);
        assert!(result.is_ok());

        if let Ok(expanded) = result {
            assert_eq!(
                expanded,
                Expr::List(vec![
                    Expr::Symbol("+".to_string()),
                    Expr::Number(1),
                    Expr::Number(2)
                ])
            );
        }
    }
}
