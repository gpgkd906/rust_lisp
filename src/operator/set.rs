// operator/set.rs

use crate::environment::Environment;
use crate::exception::LispError;
use crate::expression::Expr;
use crate::evaluator::Evaluator;

pub struct SetOps;

impl SetOps {
    pub fn eval_setf(args: &[Expr], env: &mut Environment) -> Result<Expr, LispError> {
        if args.len() != 2 {
            return Err(LispError::new("setf requires exactly two arguments"));
        }

        let symbol = match &args[0] {
            Expr::Symbol(s) => s.clone(),
            _ => return Err(LispError::new("setf: first argument must be a symbol")),
        };

        let value = Evaluator::eval_tree(&args[1], env)?;
        env.set_symbol(symbol.clone(), value.clone());
        Ok(value)
    }
}
