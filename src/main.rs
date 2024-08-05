// main.rs

mod environment;
mod parser;
mod evaluator;
mod exception;
mod expression;
mod operator;

use crate::environment::Environment;
use crate::parser::Parser;
use crate::evaluator::Evaluator;
use std::io::{self, Write};

struct Lisp;

impl Lisp {
    fn initialize() -> Environment {
        Environment::initialize()
    }

    fn repl(env: &mut Environment) {
        loop {
            match Lisp::readline("lisp:> ") {
                Ok(input) => {
                    if input.trim() == "exit" {
                        break;
                    }
                    match Parser::read(&input) {
                        Ok(ast) => {
                            let result = Evaluator::eval_tree(&ast, env);
                            match result {
                                Ok(value) => Lisp::write(format!("{}\n", value)), // Now it will print correctly
                                Err(err) => eprintln!("Error: {}", err),
                            }
                        }
                        Err(err) => eprintln!("Parse Error: {}", err),
                    }
                }
                Err(err) => eprintln!("Readline Error: {}", err),
            }
        }
    }

    fn interpreter(file: &str, env: &mut Environment) {
        match std::fs::read_to_string(file) {
            Ok(contents) => {
                for line in contents.lines() {
                    match Parser::read(line) {
                        Ok(ast) => {
                            let result = Evaluator::eval_tree(&ast, env);
                            match result {
                                Ok(value) => Lisp::write(format!("{}\n", value)),
                                Err(err) => eprintln!("Error: {}", err),
                            }
                        }
                        Err(err) => eprintln!("Parse Error: {}", err),
                    }
                }
            }
            Err(err) => eprintln!("File Error: {}", err),
        }
    }

    fn readline(prompt: &str) -> Result<String, io::Error> {
        print!("{}", prompt);
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input.trim().to_string())
    }

    fn write(output: String) {
        print!("{}", output);
    }
}

fn main() {
    let mut env = Lisp::initialize(); // 初始化全局环境
    Lisp::repl(&mut env);  // 可以切换为 `Lisp::interpreter("file.Lisp", &mut env);` 进行文件解析
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_environment() {
        let env = Lisp::initialize();
        assert!(env.get_symbol("T").is_some());
        assert_eq!(env.get_symbol("NIL"), Some(&crate::expression::Expr::List(vec![])));
    }
}
