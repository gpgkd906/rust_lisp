// operator/mod.rs

pub mod arithmetic;
pub mod list;
pub mod comparison;
pub mod control;
pub mod set;
pub mod lambda;

use std::collections::HashMap;
use std::sync::Mutex;
use crate::expression::Expr;
use crate::environment::Environment;
use crate::exception::LispError;
use lazy_static::lazy_static;

// 定义操作符函数类型
type OperatorFn = fn(&[Expr], &mut Environment) -> Result<Expr, LispError>;

// 定义 OperatorRegistry 结构体
pub struct OperatorRegistry {
    operators: HashMap<String, OperatorFn>,
}

// 使用 lazy_static 定义单例 OperatorRegistry
lazy_static! {
    static ref OPERATOR_REGISTRY: Mutex<OperatorRegistry> = Mutex::new(OperatorRegistry::new());
}

impl OperatorRegistry {
    // 初始化一个新的 OperatorRegistry
    pub fn new() -> Self {
        OperatorRegistry {
            operators: HashMap::new(),
        }
    }

    // 注册一个操作符
    pub fn register(name: &str, func: OperatorFn) {
        let mut registry = OPERATOR_REGISTRY.lock().unwrap();
        registry.operators.insert(name.to_string(), func);
    }

    // 获取一个操作符
    pub fn get(name: &str) -> Option<OperatorFn> {
        let registry = OPERATOR_REGISTRY.lock().unwrap();
        registry.operators.get(name).copied()
    }
}

use arithmetic::register_arithmetic_operators;
use comparison::register_comparison_operators;
use control::register_control_operators;
use lambda::register_lambda_operators;
use list::register_list_operators;
use set::register_set_operators;

// 初始化操作符注册表
pub fn initialize() {
    register_arithmetic_operators();
    register_comparison_operators();
    register_control_operators();
    register_lambda_operators();
    register_list_operators();
    register_set_operators();
}