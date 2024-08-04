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
        
        // 解析函数名
        let name = match &args[0] {
            Expr::Symbol(s) => s.clone(),
            _ => return Err(LispError::new("Function name must be a symbol")),
        };
        
        // 解析参数列表
        let params = match &args[1] {
            Expr::List(params) => {
                params.iter().map(|p| {
                    if let Expr::Symbol(s) = p {
                        Ok(s.clone())
                    } else {
                        Err(LispError::new("Parameter name must be a symbol"))
                    }
                }).collect::<Result<Vec<String>, LispError>>()?
            }
            _ => return Err(LispError::new("Function parameters must be a list")),
        };

        // 函数体
        let body = args[2].clone();

        // 将函数定义存储到环境中
        env.set_function(name, params, body);
        Ok(Expr::Symbol("ok".to_string()))
    }

    pub fn eval_function_call(name: &str, args: &[Expr], env: &mut Environment) -> Result<Expr, LispError> {
        if let Some((params, body)) = env.get_function(name) {
            if params.len() != args.len() {
                return Err(LispError::new("Argument count mismatch"));
            }

            // 创建局部环境并绑定参数
            let mut local_env: Environment = env.clone();
            for (param, arg) in params.iter().zip(args.iter()) {
                let eval_arg = Evaluator::eval_tree(arg, &mut local_env)?;
                local_env.set_symbol(param.clone(), eval_arg);
            }

            // 对函数体进行求值
            Evaluator::eval_tree(body, &mut local_env)
        } else {
            Err(LispError::new(&format!("Undefined function: {}", name)))
        }
    }
}
